# AI Model Gateway

A Rust web service that provides a REST API for AI model interactions using the OpenRouter API.

## Features

- Multiple endpoints for AI model interactions:
  - `/api/chat` for basic chat completions
  - `/api/prompt` for enhanced message formatting
- Support for OpenAI and Anthropic models
- Environment-based configuration
- Health check endpoint
- Proper error handling and validation
- Comprehensive logging

## Prerequisites

- Rust (latest stable version)
- OpenRouter API key (get one at [OpenRouter](https://openrouter.ai))

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd ai-model-switcher
```

2. Create a `.env` file in the project root:
```env
OPENROUTER_API_KEY=your-api-key-here
SERVER_HOST=0.0.0.0
SERVER_PORT=80
OPENROUTER_API_URL=https://openrouter.ai/api/v1/chat/completions
```

3. Build the project:
```bash
cargo build
```

## Usage

### Starting the Server

Run the server:
```bash
cargo run
```

The server will start at `http://0.0.0.0:80` by default.

## API Reference

### Health Check

```http
GET /
```

Returns "AI API Server is running" if the server is operational.

### Chat Completion Endpoint

The `/api/chat` endpoint provides basic chat completion functionality.

#### Request Format

```http
POST /api/chat
Content-Type: application/json

{
    "model": "openai/gpt-3.5-turbo",
    "messages": [
        {
            "role": "user",
            "content": "What is the meaning of life?"
        }
    ]
}
```

### Prompt Endpoint

The `/api/prompt` endpoint provides enhanced message formatting capabilities while maintaining compatibility with all supported models.

#### Request Format

```http
POST /api/prompt
Content-Type: application/json

{
    "model": "anthropic/claude-3-5-sonnet",
    "messages": [
        {
            "role": "system",
            "content": [
                {
                    "type": "text",
                    "text": "You are a helpful assistant."
                }
            ]
        },
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "What is the meaning of life?"
                }
            ]
        }
    ]
}
```

The endpoint supports two content formats:

1. Simple string content (same as `/api/chat`):
```json
"content": "Your message here"
```

2. Structured content with parts:
```json
"content": [
    {
        "type": "text",
        "text": "Your message here"
    }
]
```

#### Supported Message Roles

- `user`: For user messages
- `assistant`: For AI responses
- `system`: For system instructions
- `tool`: For tool responses (requires `tool_call_id`)

#### Optional Fields

- `name`: Optional identifier for the message sender
- `tool_call_id`: Required for `tool` role messages

### Supported Models

- `openai/gpt-3.5-turbo`
- `openai/gpt-4`
- `anthropic/claude-3-5-sonnet`
- `meta-llama/llama-3.2-3b-instruct:free`
- `meta-llama/llama-3.2-1b-instruct:free`

### Response Format

```json
{
    "message": {
        "role": "assistant",
        "content": "The model's response..."
    }
}
```

### Error Responses

- `400 Bad Request`: When messages array is empty or model is not specified
- `400 Bad Request`: When an unsupported model is specified
- `500 Internal Server Error`: For server-side errors

## Environment Variables

- `OPENROUTER_API_KEY`: Your OpenRouter API key (required)
- `SERVER_HOST`: Host to bind the server to (default: "0.0.0.0")
- `SERVER_PORT`: Port to bind the server to (default: 80)
- `OPENROUTER_API_URL`: OpenRouter API URL (default: "https://openrouter.ai/api/v1/chat/completions")

## Development

### Running in Development Mode

For development, you might want to use different environment variables:

```env
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

### Error Handling Best Practices

1. Always check the response status code
2. Implement proper timeout handling
3. Handle rate limiting appropriately
4. Validate input before sending to the API

## License

[MIT License](LICENSE)

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Support

For support, please open an issue in this repository.
