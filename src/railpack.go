package main

/*
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include "railpack.h"
*/
import "C"
import (
	"encoding/json"
	"unsafe"

	rl "github.com/railwayapp/railpack/core"
	rlApp "github.com/railwayapp/railpack/core/app"
)

func allocString(s string) *C.char {
	if s == "" {
		return nil
	}
	return C.CString(s)
}

func allocStringArray(strings []string) **C.char {
	if len(strings) == 0 {
		return nil
	}

	size := C.size_t(len(strings)) * C.size_t(unsafe.Sizeof(uintptr(0)))
	ptr := C.malloc(size)
	if ptr == nil {
		return nil
	}

	arr := (*[1 << 28]*C.char)(ptr)[:len(strings):len(strings)]
	for i, s := range strings {
		arr[i] = allocString(s)
	}

	return (**C.char)(ptr)
}

//export rp_generate_build_plan
func rp_generate_build_plan(config *C.RpConfig) *C.RpBuildResult {
	if config == nil {
		return nil
	}

	directory := C.GoString(config.directory)

	var envArgs []string
	if config.env_vars != nil && config.env_count > 0 {
		envVarsSlice := (*[1 << 28]*C.char)(unsafe.Pointer(config.env_vars))[:config.env_count:config.env_count]
		envArgs = make([]string, int(config.env_count))
		for i, envVar := range envVarsSlice {
			envArgs[i] = C.GoString(envVar)
		}
	} else {
		envArgs = []string{}
	}

	app, err := rlApp.NewApp(directory)
	if err != nil {
		return buildErrorResult(err.Error())
	}

	env, err := rlApp.FromEnvs(envArgs)
	if err != nil {
		return buildErrorResult(err.Error())
	}

	if config.verbose {
		env.SetVariable("MISE_VERBOSE", "1")
	}

	generateOptions := &rl.GenerateBuildPlanOptions{
		RailpackVersion:          C.GoString(config.railpack_version),
		BuildCommand:             C.GoString(config.build_command),
		StartCommand:             C.GoString(config.start_command),
		PreviousVersions:         map[string]string{},
		ConfigFilePath:           C.GoString(config.config_file_path),
		ErrorMissingStartCommand: bool(config.error_missing_start_command),
	}

	buildResult := rl.GenerateBuildPlan(app, env, generateOptions)

	return convertBuildResult(buildResult)
}

func buildErrorResult(errorMsg string) *C.RpBuildResult {
	ptr := C.malloc(C.size_t(unsafe.Sizeof(C.RpBuildResult{})))
	if ptr == nil {
		return nil
	}

	result := (*C.RpBuildResult)(ptr)
	C.memset(ptr, 0, C.size_t(unsafe.Sizeof(C.RpBuildResult{})))
	result.success = false

	logPtr := C.malloc(C.size_t(unsafe.Sizeof(C.RpLogEntry{})))
	if logPtr != nil {
		log := (*C.RpLogEntry)(logPtr)
		log.level = allocString("error")
		log.msg = allocString(errorMsg)
		result.logs = log
		result.logs_count = 1
	}

	return result
}

func convertBuildResult(br *rl.BuildResult) *C.RpBuildResult {
	ptr := C.malloc(C.size_t(unsafe.Sizeof(C.RpBuildResult{})))
	if ptr == nil {
		return nil
	}

	result := (*C.RpBuildResult)(ptr)
	C.memset(ptr, 0, C.size_t(unsafe.Sizeof(C.RpBuildResult{})))
	result.success = C.bool(br.Success)
	result.railpack_version = allocString(br.RailpackVersion)

	if br.Plan != nil {
		planJSON, err := json.Marshal(br.Plan)
		if err == nil {
			result.serialized_plan = allocString(string(planJSON))
		}
	}

	result.detected_providers = allocStringArray(br.DetectedProviders)
	result.detected_providers_count = C.size_t(len(br.DetectedProviders))

	if len(br.ResolvedPackages) > 0 {
		kvSize := C.size_t(len(br.ResolvedPackages)) * C.size_t(unsafe.Sizeof(C.RpKeyValue{}))
		kvPtr := C.malloc(kvSize)
		if kvPtr != nil {
			kvArr := (*[1 << 28]C.RpKeyValue)(kvPtr)[:len(br.ResolvedPackages):len(br.ResolvedPackages)]
			i := 0
			for k, v := range br.ResolvedPackages {
				kvArr[i].key = allocString(k)
				valueJSON, _ := json.Marshal(v)
				kvArr[i].value = allocString(string(valueJSON))
				i++
			}
			result.resolved_packages = (*C.RpKeyValue)(kvPtr)
			result.resolved_packages_count = C.size_t(len(br.ResolvedPackages))
		}
	}

	if len(br.Metadata) > 0 {
		metaSize := C.size_t(len(br.Metadata)) * C.size_t(unsafe.Sizeof(C.RpMetadata{}))
		metaPtr := C.malloc(metaSize)
		if metaPtr != nil {
			metaArr := (*[1 << 28]C.RpMetadata)(metaPtr)[:len(br.Metadata):len(br.Metadata)]
			i := 0
			for k, v := range br.Metadata {
				metaArr[i].key = allocString(k)
				valueJSON, _ := json.Marshal(v)
				metaArr[i].value_json = allocString(string(valueJSON))
				i++
			}
			result.metadata = (*C.RpMetadata)(metaPtr)
			result.metadata_count = C.size_t(len(br.Metadata))
		}
	}

	if len(br.Logs) > 0 {
		logSize := C.size_t(len(br.Logs)) * C.size_t(unsafe.Sizeof(C.RpLogEntry{}))
		logPtr := C.malloc(logSize)
		if logPtr != nil {
			logArr := (*[1 << 28]C.RpLogEntry)(logPtr)[:len(br.Logs):len(br.Logs)]
			for i, log := range br.Logs {
				logArr[i].level = allocString(string(log.Level))
				logArr[i].msg = allocString(log.Msg)
			}
			result.logs = (*C.RpLogEntry)(logPtr)
			result.logs_count = C.size_t(len(br.Logs))
		}
	}

	return result
}

//export rp_mem_free
func rp_mem_free(ptr unsafe.Pointer) {
	if ptr != nil {
		C.free(ptr)
	}
}

func main() {}
