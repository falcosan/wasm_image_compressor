export interface WorkerResponse {
  success: boolean;
  data?: Uint8Array;
  error?: string;
}

export interface Settings {
  type: string;
}

export interface WorkerRequest {
  inputFile: Uint8Array;
  inputType: string;
  outputType: string;
  settings?: Settings;
  compressionStrength: Float;
}
