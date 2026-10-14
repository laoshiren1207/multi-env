export interface RuntimeVersion {
  version: string;
  is_current: boolean;
  display_name: string;
}

export interface RuntimeInfo {
  name: string;
  display_name: string;
  icon: string;
  manager: string;
  is_available: boolean;
  current_version: string | null;
  versions: RuntimeVersion[];
}

export interface SwitchResult {
  success: boolean;
  message: string;
  runtime: string;
  from_version: string | null;
  to_version: string;
}

export interface Profile {
  id: string;
  name: string;
  java_version: string | null;
  node_version: string | null;
  python_version: string | null;
}
