package store

import (
	"heisenberg/internal"
)

// Interface to handle shards via consistent hashing
type shard struct {
	replicas map[string]*replica // Shard clients with replication management
	ring     ring                // Circular data structure for consistent hashing
}

func (s *shard) getShard(key []byte) (*replica, error) {
	id := s.ring.getNode(key)
	if id == "" {
		return nil, internal.NoShardsError(nil)
	}
	replica := s.replicas[id]
	return replica, nil
}

func (s *shard) addShard(id string) error {
	if !s.hasShard(id) {
		return internal.InvalidShardError(id, nil)
	}
	old := *s
	s.ring.addNode(id)
	return s.reshard(old)
}

func (s *shard) removeShard(id string) error {
	if !s.hasShard(id) {
		return internal.InvalidShardError(id, nil)
	}
	old := *s
	s.ring.removeNode(id)
	return s.reshard(old)
}

func (s *shard) hasShard(id string) bool {
	_, ok := s.replicas[id]
	return ok
}

func (s *shard) reshard(old shard) error {
	return nil
}

func (s *shard) getReplicas() map[string]*replica {
	return s.replicas
}

// Assumes non-erroneous usage
func (s *shard) addReplica(c *StoreClient, id string, shard string) error {
	s.replicas[shard].addReplica(c, id)
	return nil
}
