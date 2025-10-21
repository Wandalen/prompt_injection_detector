# Secret Management

This directory contains secret files for the vllm_inferencer project.

## File Naming Convention

All secret files **must** be prefixed with a hyphen (`-`) to ensure they are automatically excluded from git via `.gitignore`.

**Example:**
```bash
secret/-hf_token.sh         # HuggingFace API token
secret/-api_keys.env        # API keys (if needed in future)
```

## HuggingFace Token (Optional)

For downloading models from HuggingFace Hub, you may optionally set a HuggingFace token:

### Option 1: Environment Variable
```bash
export HF_TOKEN="your_token_here"
```

### Option 2: Token File
Create `secret/-hf_token.sh`:
```bash
#!/bin/bash
export HF_TOKEN="hf_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
```

Then source it:
```bash
source secret/-hf_token.sh
cargo run
```

## Notes

- **Public models (GPT-2) do NOT require authentication**
- Tokens are only needed for private/gated models
- Never commit secret files to git (they're in `.gitignore`)
- Use hyphen prefix (`-`) for all secret files
