//! Tests for lazy initialization and model caching

use injection_core::{detect, init};
use std::time::Instant;

#[test]
fn test_detect_works_without_init() -> anyhow::Result<()> {
    // Should work even without calling init() (lazy loading)
    let result = detect("Hello world")?;
    assert!(result == "benign" || result == "injection");
    Ok(())
}

#[test]
fn test_detect_reuses_model() -> anyhow::Result<()> {
    // First call (may be slow if model not loaded)
    let start = Instant::now();
    let _ = detect("First call")?;
    let first_duration = start.elapsed();

    println!("First call: {:?}", first_duration);

    // Second call should be MUCH faster (reusing cached model)
    let start = Instant::now();
    let _ = detect("Second call")?;
    let second_duration = start.elapsed();

    println!("Second call: {:?}", second_duration);

    // Second call should be at least 10x faster (typically 50x)
    // First call: ~2-3s, Second call: ~50ms
    assert!(
        second_duration < first_duration / 10,
        "Second call ({:?}) should be much faster than first ({:?})",
        second_duration,
        first_duration
    );

    Ok(())
}

#[test]
fn test_init_preloads_model() -> anyhow::Result<()> {
    // Pre-load the model
    init()?;

    // Now detect() should be fast immediately
    let start = Instant::now();
    let result = detect("After init")?;
    let duration = start.elapsed();

    println!("Detect after init(): {:?}", duration);

    // Should be fast (< 5s for Burn with CUDA warmup, < 500ms for ORT)
    // Burn needs time for CUDA kernel compilation on first inference
    assert!(
        duration.as_secs() < 5,
        "detect() after init() should be reasonably fast, got {:?}",
        duration
    );

    assert!(result == "benign" || result == "injection");
    Ok(())
}

#[test]
fn test_init_is_idempotent() -> anyhow::Result<()> {
    // Should be safe to call multiple times
    init()?;
    init()?;
    init()?;

    // Should still work
    let result = detect("Test idempotent")?;
    assert!(result == "benign" || result == "injection");

    Ok(())
}

#[test]
fn test_multiple_sequential_calls() -> anyhow::Result<()> {
    let test_inputs = vec![
        "What is the weather?",
        "Tell me a joke",
        "How do I cook pasta?",
        "What is 2+2?",
    ];

    for input in test_inputs {
        let result = detect(input)?;
        assert!(result == "benign" || result == "injection", "Invalid result for: {}", input);
    }

    Ok(())
}

#[test]
#[ignore = "Requires multi-threading setup"]
fn test_concurrent_detect_calls() -> anyhow::Result<()> {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    // Pre-load to avoid race on initialization
    init()?;

    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    // Spawn 4 threads making concurrent detect() calls
    for i in 0..4 {
        let success = Arc::clone(&success_count);
        let handle = thread::spawn(move || {
            let text = format!("Thread {} message", i);
            if let Ok(result) = detect(&text) {
                if result == "benign" || result == "injection" {
                    success.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // All threads should succeed
    assert_eq!(success_count.load(Ordering::SeqCst), 4);

    Ok(())
}
