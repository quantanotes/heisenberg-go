#include <iostream>
#include <thread>
#include <atomic>
#include "hnswlib/hnswlib.h"
#include "hnsw_wrapper.h"

hnswlib::SpaceInterface<float> *getSpace(int space_type, int dim)
{
    switch (space_type) {
        case 1:
            return new hnswlib::InnerProductSpace(dim);
        case 2:
            return new hnswlib::InnerProductSpace(dim);
        case 3:
            return new hnswlib::L2Space(dim);
        default:
            return nullptr;
    }
}

HNSW initHNSW(int dim, unsigned long int max, int m, int ef, int seed, int space_type)
{
    hnswlib::SpaceInterface<float> *space = getSpace(space_type, dim);
    hnswlib::HierarchicalNSW<float> *hnsw = new hnswlib::HierarchicalNSW<float>(space, max, m, ef, seed, true);
    return (HNSW)hnsw;
}

HNSW loadHNSW(char *location, int dim, int space_type)
{
    hnswlib::HierarchicalNSW<float> *hnsw;
    hnswlib::SpaceInterface<float> *space = getSpace(space_type, dim);
    try {
        hnsw = new hnswlib::HierarchicalNSW<float>(space, std::string(location), false, 0, true);
    }
    catch (const std::exception_ptr e) {
        return nullptr;
    }
    return hnsw;
}

bool saveHNSW(HNSW hnsw, char *path)
{
    try {
        ((hnswlib::HierarchicalNSW<float> *)hnsw)->saveIndex(std::string(path));
        return true;
    }
    catch (const std::exception_ptr e) {
        return false;
    }
}

void freeHNSW(HNSW hnsw)
{
    delete (hnswlib::HierarchicalNSW<float> *)hnsw;
}

void addPoint(HNSW hnsw, float *vec, unsigned long int id)
{
    ((hnswlib::HierarchicalNSW<float> *)hnsw)->addPoint(vec, id, true);
}

void deletePoint(HNSW hnsw, unsigned long int id)
{
    ((hnswlib::HierarchicalNSW<float> *)hnsw)->markDelete(id);
}

int search(HNSW hnsw, float *vec, int k, unsigned long int *ids, float *dists)
{
    std::priority_queue<std::pair<float, hnswlib::labeltype>> result;
    try {
        result = ((hnswlib::HierarchicalNSW<float> *)hnsw)->searchKnn(vec, k);
    } catch(const std::exception_ptr e) {
        return -1;
    }
    int size = result.size();
    for (int i = 0; i < size; i++) {
        ids[i] = result.top().second;
        dists[i] = result.top().first;
        result.pop();
    }
    return size;
}