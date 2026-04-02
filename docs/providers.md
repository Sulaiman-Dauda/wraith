# Providers

WRAITH supports multiple AI model providers through a unified interface. You can switch between providers seamlessly using the `--model` flag or environment variables.

## Anthropic (Default)

WRAITH's primary provider. Supports Claude Opus, Sonnet, and Haiku models.

### Setup

```sh
export ANTHROPIC_API_KEY=sk-ant-your-key-here
wraith
```

### Supported Models

- `claude-3.5-sonnet` (default)
- `claude-3.5-haiku`
- `claude-3-opus`
- `claude-3-sonnet`
- `claude-3-haiku`

## OpenAI-Compatible Providers

WRAITH supports any provider that implements the OpenAI Chat Completions API. This includes hosted services, local models, and cloud provider APIs.

### OpenAI

```sh
export OPENAI_API_KEY=sk-your-openai-key
export OPENAI_BASE_URL=https://api.openai.com/v1
wraith --model gpt-4o
```

### Local Models (Ollama)

```sh
export OPENAI_BASE_URL=http://localhost:11434/v1
export OPENAI_API_KEY=ollama  # Required but unused
wraith --model llama3.1:8b
```

### Local Models (LM Studio)

```sh
export OPENAI_BASE_URL=http://localhost:1234/v1
export OPENAI_API_KEY=lm-studio
wraith --model my-local-model
```

### Google Vertex AI

Use an OpenAI-compatible proxy like [LiteLLM](https://github.com/BerriAI/litellm):

```sh
# Start LiteLLM proxy with Vertex AI
litellm --model vertex_ai/gemini-pro --api_base http://0.0.0.0:4000

export OPENAI_BASE_URL=http://localhost:4000
export OPENAI_API_KEY=your-vertex-key
wraith --model gemini-pro
```

### AWS Bedrock

Use an OpenAI-compatible shim:

```sh
# Using aws-bedrock-proxy or similar
export OPENAI_BASE_URL=http://localhost:8080/v1
export OPENAI_API_KEY=$AWS_ACCESS_KEY_ID
wraith --model anthropic.claude-3-sonnet-20240229-v1:0
```

### Azure OpenAI

```sh
export OPENAI_BASE_URL=https://your-resource.openai.azure.com/openai/deployments/your-deployment
export OPENAI_API_KEY=your-azure-key
wraith --model gpt-4o
```

## Model Selection

### Command Line

```sh
# Use specific model for this session
wraith --model gpt-4o

# Change model during conversation
wraith
> /model gpt-4o-mini
```

### Environment Variable

```sh
# Set default model for all sessions
export WRAITH_MODEL=claude-3-opus
wraith
```

### Configuration File

```json
{
  "model": "gpt-4o",
  "providers": {
    "openai": {
      "model": "gpt-4o-mini",
      "baseUrl": "https://api.openai.com/v1",
      "maxTokens": 4096
    }
  }
}
```

## Model Aliases

WRAITH includes built-in aliases for common models:

| Alias    | Full Name           |
| -------- | ------------------- |
| `sonnet` | `claude-3.5-sonnet` |
| `opus`   | `claude-3-opus`     |
| `haiku`  | `claude-3-haiku`    |
| `gpt4o`  | `gpt-4o`            |
| `gpt4`   | `gpt-4-turbo`       |
| `gpt35`  | `gpt-3.5-turbo`     |

```sh
wraith --model opus
wraith --model gpt4o
```

## Provider-Specific Features

### Anthropic

- Native support for Claude's thinking tokens
- Optimized for coding and analysis tasks
- Built-in cost tracking with accurate token counting

### OpenAI-Compatible

- Streaming responses supported
- Custom base URLs for local/cloud models
- Automatic retries with exponential backoff

## Rate Limiting & Costs

### Monitor Usage

```sh
# Check current session costs
wraith
> /cost

# Real-time cost display
> /status
```

### Cost Estimation

WRAITH tracks token usage and provides cost estimates based on current provider pricing:

- **Anthropic**: Input/output token pricing
- **OpenAI**: Prompt/completion token pricing
- **Local models**: Zero cost tracking

## Authentication

### API Keys

Store API keys in:

1. Environment variables (recommended)
2. Configuration files (less secure)
3. Interactive prompt (temporary)

### OAuth (Legacy)

Previous OAuth integration has been removed. WRAITH now uses direct API key authentication for all providers.

## Error Handling

Common provider issues and solutions:

| Error                | Solution                         |
| -------------------- | -------------------------------- |
| `401 Unauthorized`   | Check API key validity           |
| `429 Rate Limited`   | Wait or upgrade plan             |
| `Connection refused` | Verify base URL for local models |
| `Model not found`    | Check model name/alias           |

## Provider Selection Strategy

1. **Default**: Anthropic Claude 3.5 Sonnet for balanced performance
2. **Code-heavy**: Claude Opus for complex refactoring/analysis
3. **Quick tasks**: Claude Haiku or GPT-3.5 for speed/cost
4. **Local development**: Ollama for privacy/offline work
5. **Enterprise**: Vertex AI or Bedrock for compliance
