# vllm_server

HTTP API server for vLLM inference engine.

## features

- **REST API**: Simple HTTP interface for text generation
- **async**: Built with Tokio and Axum for high concurrency
- **health checks**: `/health` endpoint for monitoring
- **logging**: Structured logging with tracing
- **CORS**: Cross-origin support for web clients

## endpoints

### GET /health

Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "model": "microsoft/phi-1_5",
  "version": "0.1.0"
}
```

### POST /generate

Generate text from a prompt.

**Request:**
```json
{
  "prompt": "Once upon a time",
  "max_tokens": 50
}
```

**Response:**
```json
{
  "generated": "Once upon a time, in a land far away..."
}
```

## usage

### start server

```bash
# Development mode
cargo run -p vllm_server

# Release mode (faster inference)
cargo run -p vllm_server --release
```

Server starts on `http://0.0.0.0:3000`

### test endpoints

```bash
# Health check
curl http://localhost:3000/health

# Generate text
curl -X POST http://localhost:3000/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "The future of AI", "max_tokens": 100}'

# With custom token count
curl -X POST http://localhost:3000/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Hello world", "max_tokens": 20}'
```

## configuration

### environment variables

- `RUST_LOG` - Set log level (e.g., `RUST_LOG=vllm_server=debug`)
- `HF_TOKEN` - HuggingFace token for model downloads

### secrets

Place HuggingFace token in workspace root:
- `secret/hf_token.txt`
- `secret/huggingface_token.txt`

## requirements

- NVIDIA GPU with CUDA 11.0+ (CPU fallback available)
- ~2GB GPU memory for Phi-1.5 model
- Internet connection for first run (downloads model)

## performance

- **Concurrent requests**: Handles multiple requests with async queue
- **Model sharing**: Single model instance shared across all requests
- **GPU utilization**: Maximizes GPU usage with batched inference (future)

## future enhancements

- [ ] Request batching for higher throughput
- [ ] Streaming responses (Server-Sent Events)
- [ ] Authentication/API keys
- [ ] Rate limiting
- [ ] Metrics endpoint (Prometheus)
- [ ] Model hot-swapping
- [ ] Multiple model support
