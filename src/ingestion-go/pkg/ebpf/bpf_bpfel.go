package ebpf

type bpfSpecs struct{}

type bpfPrograms struct{}

type bpfMaps struct{}

type bpfObjects struct {
	bpfPrograms
	bpfMaps
}

func loadBpfObjects(obj *bpfObjects, qopts interface{}) error {
	return nil
}
