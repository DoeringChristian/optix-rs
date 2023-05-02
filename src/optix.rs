use std::ffi::c_void;
use std::path::Path;

use cuda_rs::*;

use crate::OptixError;
include!("autogen_optix.rs");

const OPTIX_SBT_RECORD_HEADER_SIZE: usize = 32;

pub struct OptixApi {
    lib: libloading::Library,
    ftable: OptixFunctionTable,
}

impl OptixApi {
    pub unsafe fn new<P>(path: P) -> Result<Self, OptixError>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let lib = libloading::Library::new(path).unwrap();
        unsafe {
            let mut query_table: libloading::Symbol<OptixQueryFunctionTable_t> =
                lib.get(b"optixQueryFunctionTable").unwrap();
            let mut ftable = OptixFunctionTable::default();
            query_table.unwrap()(
                OPTIX_ABI_VERSION as _,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut ftable as *mut _ as *mut c_void,
                std::mem::size_of::<OptixFunctionTable>(),
            )
            .check()?;

            Ok(Self { lib, ftable })
        }
    }
}

impl OptixApi {
    pub unsafe fn optixGetErrorName(&self, result: OptixResult) -> *const ::std::os::raw::c_char {
        self.ftable.optixGetErrorName.unwrap()(result)
    }
    pub unsafe fn optixGetErrorString(&self, result: OptixResult) -> *const ::std::os::raw::c_char {
        self.ftable.optixGetErrorString.unwrap()(result)
    }
    pub unsafe fn optixDeviceContextCreate(
        &self,
        fromContext: CUcontext,
        options: *const OptixDeviceContextOptions,
        context: *mut OptixDeviceContext,
    ) -> OptixResult {
        self.ftable.optixDeviceContextCreate.unwrap()(fromContext, options, context)
    }
    pub unsafe fn optixDeviceContextDestroy(&self, context: OptixDeviceContext) -> OptixResult {
        self.ftable.optixDeviceContextDestroy.unwrap()(context)
    }
    pub unsafe fn optixDeviceContextGetPropery(
        &self,
        context: OptixDeviceContext,
        propery: OptixDeviceProperty,
        value: *mut ::std::os::raw::c_void,
        sizeInBytes: usize,
    ) -> OptixResult {
        self.ftable.optixDeviceContextGetProperty.unwrap()(context, propery, value, sizeInBytes)
    }
    pub unsafe fn optixDeviceContextSetLogCallback(
        &self,
        context: OptixDeviceContext,
        callbackFunction: OptixLogCallback,
        callbackData: *mut ::std::os::raw::c_void,
        callbackLevel: ::std::os::raw::c_uint,
    ) -> OptixResult {
        self.ftable.optixDeviceContextSetLogCallback.unwrap()(
            context,
            callbackFunction,
            callbackData,
            callbackLevel,
        )
    }
    pub unsafe fn optixDeviceContextSetCacheEnabled(
        &self,
        context: OptixDeviceContext,
        enabled: ::std::os::raw::c_int,
    ) -> OptixResult {
        self.ftable.optixDeviceContextSetCacheEnabled.unwrap()(context, enabled)
    }

    pub unsafe fn optixDeviceContextSetCacheLocation(
        &self,
        context: OptixDeviceContext,
        location: *const ::std::os::raw::c_char,
    ) -> OptixResult {
        self.ftable.optixDeviceContextSetCacheLocation.unwrap()(context, location)
    }
    pub unsafe fn optixDeviceContextSetCacheDatabaseSizes(
        &self,
        context: OptixDeviceContext,
        lowWaterMark: usize,
        highWaterMark: usize,
    ) -> OptixResult {
        self.ftable.optixDeviceContextSetCacheDatabaseSizes.unwrap()(
            context,
            lowWaterMark,
            highWaterMark,
        )
    }
    pub unsafe fn optixDeviceContextGetCacheEnabled(
        &self,
        context: OptixDeviceContext,
        enabled: *mut ::std::os::raw::c_int,
    ) -> OptixResult {
        self.ftable.optixDeviceContextGetCacheEnabled.unwrap()(context, enabled)
    }
    pub unsafe fn optixDeviceContextGetCacheLocation(
        &self,
        context: OptixDeviceContext,
        location: *mut ::std::os::raw::c_char,
        locationSize: usize,
    ) -> OptixResult {
        self.ftable.optixDeviceContextGetCacheLocation.unwrap()(context, location, locationSize)
    }
    pub unsafe fn optixDeviceContextGetCacheDatabaseSizes(
        &self,
        context: OptixDeviceContext,
        lowWaterMark: *mut usize,
        highWaterMark: *mut usize,
    ) -> OptixResult {
        self.ftable.optixDeviceContextGetCacheDatabaseSizes.unwrap()(
            context,
            lowWaterMark,
            highWaterMark,
        )
    }
    pub unsafe fn optixModuleCreateFromPTX(
        &self,
        context: OptixDeviceContext,
        moduleCompileOptions: *const OptixModuleCompileOptions,
        pipelineCompileOptions: *const OptixPipelineCompileOptions,
        PTX: *const ::std::os::raw::c_char,
        PTXsize: usize,
        logString: *mut ::std::os::raw::c_char,
        logStringSize: *mut usize,
        module: *mut OptixModule,
    ) -> OptixResult {
        self.ftable.optixModuleCreateFromPTX.unwrap()(
            context,
            moduleCompileOptions,
            pipelineCompileOptions,
            PTX,
            PTXsize,
            logString,
            logStringSize,
            module,
        )
    }
    pub unsafe fn optixModuleCreateFromPTXWithTasks(
        &self,
        context: OptixDeviceContext,
        moduleCompileOptions: *const OptixModuleCompileOptions,
        pipelineCompileOptions: *const OptixPipelineCompileOptions,
        PTX: *const ::std::os::raw::c_char,
        PTXsize: usize,
        logString: *mut ::std::os::raw::c_char,
        logStringSize: *mut usize,
        module: *mut OptixModule,
        firstTask: *mut OptixTask,
    ) -> OptixResult {
        self.ftable.optixModuleCreateFromPTXWithTasks.unwrap()(
            context,
            moduleCompileOptions,
            pipelineCompileOptions,
            PTX,
            PTXsize,
            logString,
            logStringSize,
            module,
            firstTask,
        )
    }
    pub unsafe fn optixModuleGetCompilationState(
        &self,
        module: OptixModule,
        state: *mut OptixModuleCompileState,
    ) -> OptixResult {
        self.ftable.optixModuleGetCompilationState.unwrap()(module, state)
    }
    pub unsafe fn optixModuleDestroy(&self, module: OptixModule) -> OptixResult {
        self.ftable.optixModuleDestroy.unwrap()(module)
    }

    pub unsafe fn optixBuiltinISModuleGet(
        &self,
        context: OptixDeviceContext,
        moduleCompileOptions: *const OptixModuleCompileOptions,
        pipelineCompileOptions: *const OptixPipelineCompileOptions,
        builtinISOptions: *const OptixBuiltinISOptions,
        builtinModule: *mut OptixModule,
    ) -> OptixResult {
        self.ftable.optixBuiltinISModuleGet.unwrap()(
            context,
            moduleCompileOptions,
            pipelineCompileOptions,
            builtinISOptions,
            builtinModule,
        )
    }
    pub unsafe fn optixTaskExecute(
        &self,
        task: OptixTask,
        additionalTasks: *mut OptixTask,
        maxNumAdditionalTasks: ::std::os::raw::c_uint,
        numAdditionalTasksCreated: *mut ::std::os::raw::c_uint,
    ) -> OptixResult {
        self.ftable.optixTaskExecute.unwrap()(
            task,
            additionalTasks,
            maxNumAdditionalTasks,
            numAdditionalTasksCreated,
        )
    }

    pub unsafe fn optixProgramGroupCreate(
        &self,
        context: OptixDeviceContext,
        programDescriptions: *const OptixProgramGroupDesc,
        numProgramGroups: ::std::os::raw::c_uint,
        options: *const OptixProgramGroupOptions,
        logString: *mut ::std::os::raw::c_char,
        logStringSize: *mut usize,
        programGroups: *mut OptixProgramGroup,
    ) -> OptixResult {
        self.ftable.optixProgramGroupCreate.unwrap()(
            context,
            programDescriptions,
            numProgramGroups,
            options,
            logString,
            logStringSize,
            programGroups,
        )
    }
    pub unsafe fn optixProgramGroupDestroy(&self, programGroup: OptixProgramGroup) -> OptixResult {
        self.ftable.optixProgramGroupDestroy.unwrap()(programGroup)
    }

    pub unsafe fn optixProgramGroupGetStackSize(
        &self,
        programGroup: OptixProgramGroup,
        stackSizes: *mut OptixStackSizes,
    ) -> OptixResult {
        self.ftable.optixProgramGroupGetStackSize.unwrap()(programGroup, stackSizes)
    }
    pub unsafe fn optixPipelineCreate(
        &self,
        context: OptixDeviceContext,
        pipelineCompileOptions: *const OptixPipelineCompileOptions,
        pipelineLinkOptions: *const OptixPipelineLinkOptions,
        programGroups: *const OptixProgramGroup,
        numProgramGroups: ::std::os::raw::c_uint,
        logString: *mut ::std::os::raw::c_char,
        logStringSize: *mut usize,
        pipeline: *mut OptixPipeline,
    ) -> OptixResult {
        self.ftable.optixPipelineCreate.unwrap()(
            context,
            pipelineCompileOptions,
            pipelineLinkOptions,
            programGroups,
            numProgramGroups,
            logString,
            logStringSize,
            pipeline,
        )
    }
    pub unsafe fn optixPipelineDestroy(&self, pipeline: OptixPipeline) -> OptixResult {
        self.ftable.optixPipelineDestroy.unwrap()(pipeline)
    }
    pub unsafe fn optixPipelineSetStackSize(
        &self,
        pipeline: OptixPipeline,
        directCallableStackSizeFromTraversal: ::std::os::raw::c_uint,
        directCallableStackSizeFromState: ::std::os::raw::c_uint,
        continuationStackSize: ::std::os::raw::c_uint,
        maxTraversableGraphDepth: ::std::os::raw::c_uint,
    ) -> OptixResult {
        self.ftable.optixPipelineSetStackSize.unwrap()(
            pipeline,
            directCallableStackSizeFromTraversal,
            directCallableStackSizeFromState,
            continuationStackSize,
            maxTraversableGraphDepth,
        )
    }
    pub unsafe fn optixAccelComputeMemoryUsage(
        &self,
        context: OptixDeviceContext,
        accelOptions: *const OptixAccelBuildOptions,
        buildInputs: *const OptixBuildInput,
        numBuildInputs: ::std::os::raw::c_uint,
        bufferSizes: *mut OptixAccelBufferSizes,
    ) -> OptixResult {
        self.ftable.optixAccelComputeMemoryUsage.unwrap()(
            context,
            accelOptions,
            buildInputs,
            numBuildInputs,
            bufferSizes,
        )
    }
    pub unsafe fn optixAccelBuild(
        &self,
        context: OptixDeviceContext,
        stream: CUstream,
        accelOptions: *const OptixAccelBuildOptions,
        buildInputs: *const OptixBuildInput,
        numBuildInputs: ::std::os::raw::c_uint,
        tempBuffer: CUdeviceptr,
        tempBufferSizeInBytes: usize,
        outputBuffer: CUdeviceptr,
        outputBufferSizeInBytes: usize,
        outputHandle: *mut OptixTraversableHandle,
        emittedProperties: *const OptixAccelEmitDesc,
        numEmittedProperties: ::std::os::raw::c_uint,
    ) -> OptixResult {
        self.ftable.optixAccelBuild.unwrap()(
            context,
            stream,
            accelOptions,
            buildInputs,
            numBuildInputs,
            tempBuffer,
            tempBufferSizeInBytes,
            outputBuffer,
            outputBufferSizeInBytes,
            outputHandle,
            emittedProperties,
            numEmittedProperties,
        )
    }
    pub unsafe fn optixAccelGetRelocationInfo(
        &self,
        context: OptixDeviceContext,
        handle: OptixTraversableHandle,
        info: *mut OptixAccelRelocationInfo,
    ) -> OptixResult {
        self.ftable.optixAccelGetRelocationInfo.unwrap()(context, handle, info)
    }
    pub unsafe fn optixAccelCheckRelocationCompatibility(
        &self,
        context: OptixDeviceContext,
        info: *const OptixAccelRelocationInfo,
        compatible: *mut ::std::os::raw::c_int,
    ) -> OptixResult {
        self.ftable.optixAccelCheckRelocationCompatibility.unwrap()(context, info, compatible)
    }
    pub unsafe fn optixAccelRelocate(
        &self,
        context: OptixDeviceContext,
        stream: CUstream,
        info: *const OptixAccelRelocationInfo,
        instanceTraversableHandles: CUdeviceptr,
        numInstanceTraversableHandles: usize,
        targetAccel: CUdeviceptr,
        targetAccelSizeInBytes: usize,
        targetHandle: *mut OptixTraversableHandle,
    ) -> OptixResult {
        self.ftable.optixAccelRelocate.unwrap()(
            context,
            stream,
            info,
            instanceTraversableHandles,
            numInstanceTraversableHandles,
            targetAccel,
            targetAccelSizeInBytes,
            targetHandle,
        )
    }
    pub unsafe fn optixAccelCompact(
        &self,
        context: OptixDeviceContext,
        stream: CUstream,
        inputHandle: OptixTraversableHandle,
        outputBuffer: CUdeviceptr,
        outputBufferSizeInBytes: usize,
        outputHandle: *mut OptixTraversableHandle,
    ) -> OptixResult {
        self.ftable.optixAccelCompact.unwrap()(
            context,
            stream,
            inputHandle,
            outputBuffer,
            outputBufferSizeInBytes,
            outputHandle,
        )
    }
    pub unsafe fn optixConvertPointerToTraversableHandle(
        &self,
        onDevice: OptixDeviceContext,
        pointer: CUdeviceptr,
        traversableType: OptixTraversableType,
        traversableHandle: *mut OptixTraversableHandle,
    ) -> OptixResult {
        self.ftable.optixConvertPointerToTraversableHandle.unwrap()(
            onDevice,
            pointer,
            traversableType,
            traversableHandle,
        )
    }
    pub unsafe fn optixSbtRecordPackHeader(
        &self,
        programGroup: OptixProgramGroup,
        sbtRecordHeaderHostPointer: *mut ::std::os::raw::c_void,
    ) -> OptixResult {
        self.ftable.optixSbtRecordPackHeader.unwrap()(programGroup, sbtRecordHeaderHostPointer)
    }
    pub unsafe fn optixLaunch(
        &self,
        pipeline: OptixPipeline,
        stream: CUstream,
        pipelineParams: CUdeviceptr,
        pipelineParamsSize: usize,
        sbt: *const OptixShaderBindingTable,
        width: ::std::os::raw::c_uint,
        height: ::std::os::raw::c_uint,
        depth: ::std::os::raw::c_uint,
    ) -> OptixResult {
        self.ftable.optixLaunch.unwrap()(
            pipeline,
            stream,
            pipelineParams,
            pipelineParamsSize,
            sbt,
            width,
            height,
            depth,
        )
    }
    pub unsafe fn optixDenoiserCreate(
        &self,
        context: OptixDeviceContext,
        modelKind: OptixDenoiserModelKind,
        options: *const OptixDenoiserOptions,
        returnHandle: *mut OptixDenoiser,
    ) -> OptixResult {
        self.ftable.optixDenoiserCreate.unwrap()(context, modelKind, options, returnHandle)
    }
    pub unsafe fn optixDenoiserDestroy(&self, handle: OptixDenoiser) -> OptixResult {
        self.ftable.optixDenoiserDestroy.unwrap()(handle)
    }
    pub unsafe fn optixDenoiserComputeMemoryResources(
        &self,
        handle: OptixDenoiser,
        maximumInputWidth: ::std::os::raw::c_uint,
        maximumInputHeight: ::std::os::raw::c_uint,
        returnSizes: *mut OptixDenoiserSizes,
    ) -> OptixResult {
        self.ftable.optixDenoiserComputeMemoryResources.unwrap()(
            handle,
            maximumInputWidth,
            maximumInputHeight,
            returnSizes,
        )
    }
    pub unsafe fn optixDenoiserSetup(
        &self,
        denoiser: OptixDenoiser,
        stream: CUstream,
        inputWidth: ::std::os::raw::c_uint,
        inputHeight: ::std::os::raw::c_uint,
        state: CUdeviceptr,
        stateSizeInBytes: usize,
        scratch: CUdeviceptr,
        scratchSizeInBytes: usize,
    ) -> OptixResult {
        self.ftable.optixDenoiserSetup.unwrap()(
            denoiser,
            stream,
            inputWidth,
            inputHeight,
            state,
            stateSizeInBytes,
            scratch,
            scratchSizeInBytes,
        )
    }
    pub unsafe fn optixDenoiserInvoke(
        &self,
        denoiser: OptixDenoiser,
        stream: CUstream,
        params: *const OptixDenoiserParams,
        denoiserState: CUdeviceptr,
        denoiserStateSizeInBytes: usize,
        guideLayer: *const OptixDenoiserGuideLayer,
        layers: *const OptixDenoiserLayer,
        numLayers: ::std::os::raw::c_uint,
        inputOffsetX: ::std::os::raw::c_uint,
        inputOffsetY: ::std::os::raw::c_uint,
        scratch: CUdeviceptr,
        scratchSizeInBytes: usize,
    ) -> OptixResult {
        self.ftable.optixDenoiserInvoke.unwrap()(
            denoiser,
            stream,
            params,
            denoiserState,
            denoiserStateSizeInBytes,
            guideLayer,
            layers,
            numLayers,
            inputOffsetX,
            inputOffsetY,
            scratch,
            scratchSizeInBytes,
        )
    }
    pub unsafe fn optixDenoiserComputeIntensity(
        &self,
        handle: OptixDenoiser,
        stream: CUstream,
        inputImage: *const OptixImage2D,
        outputIntensity: CUdeviceptr,
        scratch: CUdeviceptr,
        scratchSizeInBytes: usize,
    ) -> OptixResult {
        self.ftable.optixDenoiserComputeIntensity.unwrap()(
            handle,
            stream,
            inputImage,
            outputIntensity,
            scratch,
            scratchSizeInBytes,
        )
    }
    pub unsafe fn optixDenoiserComputeAverageColor(
        &self,
        handle: OptixDenoiser,
        stream: CUstream,
        inputImage: *const OptixImage2D,
        outputAverageColor: CUdeviceptr,
        scratch: CUdeviceptr,
        scratchSizeInBytes: usize,
    ) -> OptixResult {
        self.ftable.optixDenoiserComputeAverageColor.unwrap()(
            handle,
            stream,
            inputImage,
            outputAverageColor,
            scratch,
            scratchSizeInBytes,
        )
    }
    pub unsafe fn optixDenoiserCreateWithUserModel(
        &self,
        context: OptixDeviceContext,
        data: *const ::std::os::raw::c_void,
        dataSizeInBytes: usize,
        returnHandle: *mut OptixDenoiser,
    ) -> OptixResult {
        self.ftable.optixDenoiserCreateWithUserModel.unwrap()(
            context,
            data,
            dataSizeInBytes,
            returnHandle,
        )
    }
}
