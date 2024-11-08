-- Iniciar la base de datos
USE NS restaurant;
USE DB restaurant;

-- Definición de la tabla de usuarios
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD email ON user TYPE string ASSERT is::email($value);
DEFINE FIELD password ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD status ON user TYPE string;
DEFINE FIELD user_type ON user TYPE string;
DEFINE FIELD is_verified ON user TYPE bool DEFAULT false;
DEFINE FIELD is_active ON user TYPE bool DEFAULT true;
DEFINE FIELD failed_login_attempts ON user TYPE int DEFAULT 0;
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD last_login ON user TYPE datetime;
DEFINE FIELD locked_until ON user TYPE datetime;

-- Índices para usuario
DEFINE INDEX idx_user_email ON user FIELDS email UNIQUE;

-- Tipos
DEFINE TYPE address SCHEMAFULL {
  street: string,
  city: string,
  state: string,
  country: string,
  postal_code: string
};

DEFINE TYPE emergency_contact SCHEMAFULL {
  name: string,
  phone: string,
  relationship: string
};

-- Definición de la tabla de perfiles
DEFINE TABLE profile SCHEMAFULL;
DEFINE FIELD user ON profile TYPE record(user);
DEFINE FIELD first_name ON profile TYPE string;
DEFINE FIELD last_name ON profile TYPE string;
DEFINE FIELD birth_date ON profile TYPE datetime;
DEFINE FIELD phone ON profile TYPE string;
DEFINE FIELD avatar ON profile TYPE option<string>;
DEFINE FIELD address ON profile TYPE option<object {
  street: string,
  city: string,
  state: string,
  country: string,
  postal_code: string
}>;
DEFINE FIELD emergency_contact ON profile TYPE option<object {
  name: string,
  phone: string,
  relationship: string
}>;
DEFINE FIELD is_active ON profile TYPE bool DEFAULT true;
DEFINE FIELD created_at ON profile TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON profile TYPE datetime DEFAULT time::now();

-- Índices para perfiles
DEFINE INDEX user_id_idx ON profile FIELDS user;

-- Definición de la tabla de roles
DEFINE TABLE role SCHEMAFULL;
DEFINE FIELD name ON role TYPE string 
    ASSERT $value INSIDE [
        'super_admin',
        'admin',
        'restaurant_manager',
        'head_chef',
        'chef',
        'cashier',
        'waiter',
        'inventory_manager',
        'customer'
    ];
DEFINE FIELD description ON role TYPE string;
DEFINE FIELD hierarchy_level ON role TYPE int;
DEFINE FIELD is_active ON role TYPE bool DEFAULT true;
DEFINE FIELD created_at ON role TYPE datetime DEFAULT time::now();

-- Índice para roles
DEFINE INDEX idx_role_name ON role FIELDS name UNIQUE;

-- Definición de la tabla de permisos
DEFINE TABLE permission SCHEMAFULL;
DEFINE FIELD name ON permission TYPE string;
DEFINE FIELD description ON permission TYPE string;
DEFINE FIELD resource ON permission TYPE string;
DEFINE FIELD is_active ON permission TYPE bool DEFAULT true;
DEFINE FIELD action ON permission TYPE string
    ASSERT $value INSIDE [
        'create',
        'read',
        'update',
        'delete',
        'approve',
        'cancel',
        'manage'
    ];

-- Índice para permisos
DEFINE INDEX idx_permission_name ON permission FIELDS name UNIQUE;

-- Definición de la tabla de relación usuario-rol
DEFINE TABLE user_role SCHEMAFULL;
DEFINE FIELD user ON user_role TYPE record(user);
DEFINE FIELD role ON user_role TYPE record(role);
DEFINE FIELD assigned_at ON user_role TYPE datetime DEFAULT time::now();
DEFINE FIELD assigned_by ON user_role TYPE record(user);

-- Índice para relación usuario-rol
DEFINE INDEX idx_user_role ON user_role FIELDS user, role UNIQUE;

-- Definición de la tabla de relación rol-permiso
DEFINE TABLE role_permission SCHEMAFULL;
DEFINE FIELD role ON role_permission TYPE record(role);
DEFINE FIELD permission ON role_permission TYPE record(permission);

-- Índice para relación rol-permiso
DEFINE INDEX idx_role_permission ON role_permission FIELDS role, permission UNIQUE;

-- Definición de la tabla de refresh tokens
DEFINE TABLE refresh_token SCHEMAFULL;
DEFINE FIELD user ON refresh_token TYPE record(user);
DEFINE FIELD token ON refresh_token TYPE string;
DEFINE FIELD access_token ON refresh_token TYPE string;
DEFINE FIELD expires_at ON refresh_token TYPE datetime;
DEFINE FIELD created_at ON refresh_token TYPE datetime DEFAULT time::now();
DEFINE FIELD used ON refresh_token TYPE bool DEFAULT false;
DEFINE FIELD invalidated ON refresh_token TYPE bool DEFAULT false;

-- Índices para refresh tokens
DEFINE INDEX idx_refresh_token ON refresh_token FIELDS token UNIQUE;
DEFINE INDEX idx_refresh_token_user ON refresh_token FIELDS user, used, invalidated;

-- Crear algunos roles y permisos iniciales
LET $timestamp = time::now();

-- Crear rol super_admin
CREATE role:super_admin SET 
  name = 'super_admin',
  description = 'Super Administrator with full access',
  hierarchy_level = 1,
  created_at = $timestamp;

-- Crear rol admin
CREATE role:admin SET 
  name = 'admin',
  description = 'Administrator with limited access',
  hierarchy_level = 2,
  created_at = $timestamp;

-- Crear rol customer
CREATE role:customer SET 
  name = 'customer',
  description = 'Customer with limited access',
  hierarchy_level = 3,
  created_at = $timestamp;

-- Crear permisos básicos
CREATE permission SET 
  name = 'users.manage',
  description = 'Manage users',
  resource = 'users',
  action = 'manage';

CREATE permission SET 
  name = 'roles.manage',
  description = 'Manage roles',
  resource = 'users',
  action = 'manage';

CREATE permission SET 
  name = 'permissions.manage',
  description = 'Manage permissions',
  resource = 'users',
  action = 'manage';

-- Asignar permisos al rol super_admin
RELATE role:super_admin->role_permission->permission:users.manage;
RELATE role:super_admin->role_permission->permission:roles.manage;
RELATE role:super_admin->role_permission->permission:permissions.manage;