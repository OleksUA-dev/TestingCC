// Entity and Attribute types
export interface Entity {
  id: string;
  system_name: string;
  display_name: string;
  display_name_plural: string;
  description?: string;
  is_system: boolean;
  is_active: boolean;
  icon?: string;
  created_at: string;
  updated_at: string;
}

export type DataType =
  | 'string'
  | 'text'
  | 'integer'
  | 'float'
  | 'decimal'
  | 'boolean'
  | 'date_time'
  | 'date'
  | 'lookup'
  | 'enum';

export interface Attribute {
  id: string;
  entity_id: string;
  system_name: string;
  display_name: string;
  description?: string;
  data_type: DataType;
  is_required: boolean;
  is_unique: boolean;
  is_system: boolean;
  is_active: boolean;
  default_value?: any;
  lookup_entity_id?: string;
  enum_values?: EnumValue[];
  validation_rules?: any;
  display_order: number;
  created_at: string;
  updated_at: string;
}

export interface EnumValue {
  key: string;
  value: string;
  display_order: number;
}

// UI Metadata types
export interface FormLayout {
  id: string;
  entity_id: string;
  name: string;
  is_default: boolean;
  sections: FormSection[];
  created_at: string;
  updated_at: string;
}

export interface FormSection {
  title: string;
  display_order: number;
  columns: number;
  fields: FormField[];
}

export interface FormField {
  attribute_id: string;
  label?: string;
  placeholder?: string;
  help_text?: string;
  display_order: number;
  is_readonly: boolean;
  is_visible: boolean;
  column_span: number;
}

export interface GridLayout {
  id: string;
  entity_id: string;
  name: string;
  is_default: boolean;
  columns: GridColumn[];
  default_sort_by?: string;
  default_sort_order?: 'asc' | 'desc';
  page_size: number;
  created_at: string;
  updated_at: string;
}

export interface GridColumn {
  attribute_id: string;
  header?: string;
  width?: number;
  display_order: number;
  is_sortable: boolean;
  is_filterable: boolean;
}

export interface EntityMetadata {
  entity: Entity;
  attributes: Attribute[];
  form_layouts: FormLayout[];
  grid_layouts: GridLayout[];
}

// Authentication types
export interface User {
  id: string;
  username: string;
  email: string;
  is_active: boolean;
  is_admin: boolean;
  created_at: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  access_token: string;
  token_type: string;
  expires_in: number;
  user: User;
}

// Data types
export interface Record {
  id: string;
  [key: string]: any;
}

export interface ListResponse<T> {
  data: T[];
  total: number;
}
