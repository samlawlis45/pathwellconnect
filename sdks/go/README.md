# Pathwell Connect Go SDK

Go SDK for Pathwell Connect MVG.

## Installation

```bash
go mod download
```

## Usage

```go
package main

import (
    "fmt"
    "io"
    "github.com/pathwell/connect-go/pathwell"
)

func main() {
    // Initialize client
    client, err := pathwell.NewClient(pathwell.ClientOptions{
        AgentID:        "agent-123",
        PrivateKeyPath: "./agent.key",
        ProxyURL:       "https://proxy.pathwell.io",
    })
    if err != nil {
        panic(err)
    }

    // Make requests
    resp, err := client.Post(
        "https://api.example.com/v1/chat",
        map[string]string{"Content-Type": "application/json"},
        map[string]interface{}{"message": "Hello"},
    )
    if err != nil {
        panic(err)
    }
    defer resp.Body.Close()

    body, _ := io.ReadAll(resp.Body)
    fmt.Println(string(body))
}
```

## Generating Keys

```go
import "github.com/pathwell/connect-go/pathwell"

keyPair, err := pathwell.GenerateKeyPair()
if err != nil {
    panic(err)
}

// Save keys
os.WriteFile("agent.key", []byte(keyPair.PrivateKey), 0600)
os.WriteFile("agent.pub", []byte(keyPair.PublicKey), 0644)
```

## API Reference

### Client

Main client struct for making authenticated requests.

#### Constructor

```go
NewClient(options ClientOptions) (*Client, error)
```

#### Methods

- `Call(method, url, headers, body)`: Make a request
- `Get(url, headers)`: GET request
- `Post(url, headers, body)`: POST request
- `Put(url, headers, body)`: PUT request
- `Patch(url, headers, body)`: PATCH request
- `Delete(url, headers)`: DELETE request

