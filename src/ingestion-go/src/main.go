package main

import (
	"fmt"
	"log"
	"time"
)

func main() {
	targetAddr := "127.0.0.1:50051"

	client, err := NewAnalysisClient(targetAddr)
	if err != nil {
		log.Fatalf("Failed to connect to gRPC server: %v", err)
	}
	defer client.Close()

	fmt.Printf("Ingestion service started. Sending mock events to %s...\n", targetAddr)

	for {
		resp, err := client.SendNetworkEvent("GET", "/api/v1/debug?file=../../etc/passwd", []byte(`{"status":"ok"}`))
		if err != nil {
			log.Printf("Error sending event: %v", err)
		} else {
			log.Printf("Server response: SessionID=%s, Status=%v", resp.GetSessionId(), resp.GetStatus())
		}

		time.Sleep(2 * time.Second)
	}
}
