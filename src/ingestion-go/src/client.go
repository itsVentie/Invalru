package main

import (
	"context"
	"time"

	pb "ingestion-go/pkg/pb"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

type AnalysisClient struct {
	conn   *grpc.ClientConn
	client pb.AnalysisServiceClient
}

func NewAnalysisClient(addr string) (*AnalysisClient, error) {
	conn, err := grpc.Dial(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, err
	}

	client := pb.NewAnalysisServiceClient(conn)
	return &AnalysisClient{conn: conn, client: client}, nil
}

func (c *AnalysisClient) Close() error {
	return c.conn.Close()
}

func (c *AnalysisClient) SendNetworkEvent(method, url string, body []byte) (*pb.AnalysisResponse, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()

	event := &pb.NetworkEvent{
		Timestamp: time.Now().Unix(),
		SourceIp:  "127.0.0.1",
		Method:    method,
		Url:       url,
		RawBody:   body,
	}

	return c.client.PushEvent(ctx, event)
}
