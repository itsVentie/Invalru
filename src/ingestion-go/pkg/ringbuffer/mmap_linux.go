//go:build linux
// +build linux

package ringbuffer

import (
	"os"

	"golang.org/x/sys/unix"
)

type MmapBuffer struct {
	fd   int
	data []byte
}

func NewMmapBuffer(file *os.File, size int) (*MmapBuffer, error) {
	fd := int(file.Fd())
	data, err := unix.Mmap(fd, 0, size, unix.PROT_READ|unix.PROT_WRITE, unix.MAP_SHARED)
	if err != nil {
		return nil, err
	}
	return &MmapBuffer{fd: fd, data: data}, nil
}
