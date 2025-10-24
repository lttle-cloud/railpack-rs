#ifndef RAILPACK_H
#define RAILPACK_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    const char* key;
    const char* value;
} RpKeyValue;

typedef struct {
    const char* directory;

    const char** env_vars;
    size_t env_count;

    bool verbose;

    const char* railpack_version;
    const char* build_command;
    const char* start_command;
    const char* config_file_path;
    bool error_missing_start_command;
} RpConfig;

typedef struct {
    const char* level;
    const char* msg;
} RpLogEntry;

typedef struct {
    const char* key;
    const char* value_json;
} RpMetadata;

typedef struct {
    bool success;

    const char* railpack_version;

    const char* serialized_plan;

    const char** detected_providers;
    size_t detected_providers_count;

    RpKeyValue* resolved_packages;
    size_t resolved_packages_count;

    RpMetadata* metadata;
    size_t metadata_count;

    RpLogEntry* logs;
    size_t logs_count;
} RpBuildResult;

RpBuildResult* rp_generate_build_plan(RpConfig* config);

void rp_mem_free(void* ptr);

#ifdef __cplusplus
}
#endif

#endif // RAILPACK_H    
