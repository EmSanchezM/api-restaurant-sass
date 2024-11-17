-- Iniciar la base de datos
USE NS restaurants;
USE DB restaurants;

-- Definición de la tabla de usuarios
DEFINE TABLE users SCHEMAFULL;
DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value);
DEFINE FIELD password ON users TYPE string ASSERT $value != NONE;
DEFINE FIELD status ON users TYPE string;
DEFINE FIELD user_type ON users TYPE string;
DEFINE FIELD profile ON users TYPE option<record<profiles>>;
DEFINE FIELD is_verified ON users TYPE bool DEFAULT false;
DEFINE FIELD is_active ON users TYPE bool DEFAULT true;
DEFINE FIELD failed_login_attempts ON users TYPE int DEFAULT 0;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD last_login ON users TYPE option<datetime>;
DEFINE FIELD locked_until ON users TYPE option<datetime>;

-- Índices para usuario
DEFINE INDEX idx_user_email ON users FIELDS email UNIQUE;

-- Definición de la tabla de perfiles
DEFINE TABLE profiles SCHEMAFULL;
DEFINE FIELD user ON profiles TYPE record<users> ASSERT $value != NONE;
DEFINE FIELD first_name ON profiles TYPE string;
DEFINE FIELD last_name ON profiles TYPE string;
DEFINE FIELD birth_date ON profiles TYPE datetime;
DEFINE FIELD phone ON profiles TYPE string;
DEFINE FIELD avatar ON profiles TYPE option<string>;
DEFINE FIELD address ON profiles TYPE option<object>;
DEFINE FIELD address.street ON profiles TYPE string;
DEFINE FIELD address.city ON profiles TYPE string;
DEFINE FIELD address.state ON profiles TYPE string;
DEFINE FIELD address.country ON profiles TYPE string;
DEFINE FIELD address.postal_code ON profiles TYPE string;
DEFINE FIELD emergency_contact ON profiles TYPE option<object>;
DEFINE FIELD emergency_contact.name ON profiles TYPE string;
DEFINE FIELD emergency_contact.phone ON profiles TYPE string;
DEFINE FIELD emergency_contact.relationship ON profiles TYPE string;
DEFINE FIELD is_active ON profiles TYPE bool DEFAULT true;
DEFINE FIELD created_at ON profiles TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON profiles TYPE datetime DEFAULT time::now();

-- Índices para perfiles
DEFINE INDEX idx_profile_user ON profiles FIELDS user UNIQUE;

-- Definición de la tabla de roles
DEFINE TABLE roles SCHEMAFULL;
DEFINE FIELD name ON roles TYPE string 
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
DEFINE FIELD description ON roles TYPE string;
DEFINE FIELD hierarchy_level ON roles TYPE int;
DEFINE FIELD is_active ON roles TYPE bool DEFAULT true;
DEFINE FIELD created_at ON roles TYPE datetime DEFAULT time::now();

-- Índice para roles
DEFINE INDEX idx_role_name ON roles FIELDS name UNIQUE;

-- Definición de la tabla de permisos
DEFINE TABLE permissions SCHEMAFULL;
DEFINE FIELD name ON permissions TYPE string;
DEFINE FIELD description ON permissions TYPE string;
DEFINE FIELD resource ON permissions TYPE string;
DEFINE FIELD is_active ON permissions TYPE bool DEFAULT true;
DEFINE FIELD action ON permissions TYPE string
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
DEFINE INDEX idx_permission_name ON permissions FIELDS name UNIQUE;

-- Definición de la tabla de relación usuario-rol
DEFINE TABLE users_roles SCHEMAFULL;
DEFINE FIELD user ON users_roles TYPE record<users>;
DEFINE FIELD role ON users_roles TYPE record<roles>;
DEFINE FIELD assigned_by ON users_roles TYPE option<record<users>>;
DEFINE FIELD assigned_at ON users_roles TYPE datetime DEFAULT time::now();

-- Índice para relación usuario-rol
DEFINE INDEX idx_user_role ON users_roles FIELDS user, role UNIQUE;

-- Definición de la tabla de relación rol-permiso
DEFINE TABLE roles_permissions SCHEMAFULL;
DEFINE FIELD role ON roles_permissions TYPE record<roles>;
DEFINE FIELD permission ON roles_permissions TYPE record<permissions>;

-- Índice para relación rol-permiso
DEFINE INDEX idx_role_permission ON roles_permissions FIELDS role, permission UNIQUE;

-- Definición de la tabla de refresh tokens
DEFINE TABLE refresh_tokens SCHEMAFULL;
DEFINE FIELD user ON refresh_tokens TYPE record<users>;
DEFINE FIELD token ON refresh_tokens TYPE string;
DEFINE FIELD access_token ON refresh_tokens TYPE string;
DEFINE FIELD expires_at ON refresh_tokens TYPE datetime;
DEFINE FIELD created_at ON refresh_tokens TYPE datetime DEFAULT time::now();
DEFINE FIELD used ON refresh_tokens TYPE bool DEFAULT false;
DEFINE FIELD invalidated ON refresh_tokens TYPE bool DEFAULT false;

-- Índices para refresh tokens
DEFINE INDEX idx_refresh_token ON refresh_tokens FIELDS token, user UNIQUE;
DEFINE INDEX idx_refresh_token_user ON refresh_tokens FIELDS used, invalidated;

-- Crear algunos roles y permisos iniciales
LET $timestamp = time::now();

-- Crear rol super_admin
CREATE roles:super_admin SET 
  name = 'super_admin',
  description = 'Super Administrator with full access',
  hierarchy_level = 1,
  created_at = $timestamp;

-- Crear rol admin
CREATE roles:admin SET 
  name = 'admin',
  description = 'Administrator with limited access',
  hierarchy_level = 2,
  created_at = $timestamp;

-- Crear rol customer
CREATE roles:customer SET 
  name = 'customer',
  description = 'Customer with limited access',
  hierarchy_level = 3,
  created_at = $timestamp;

-- Crear permisos básicos
CREATE permissions SET 
  name = 'users.manage',
  description = 'Manage users',
  resource = 'users',
  action = 'manage';

CREATE permissions SET 
  name = 'roles.manage',
  description = 'Manage roles',
  resource = 'users',
  action = 'manage';

CREATE permissions SET 
  name = 'permissions.manage',
  description = 'Manage permissions',
  resource = 'users',
  action = 'manage';

CREATE roles_permissions SET
  role = roles:super_admin,
  permission = (SELECT id FROM permissions WHERE name = 'users.manage' LIMIT 1)[0].id;

-- Crear el usuario super admin
CREATE users SET 
  email = 'emsanchez891@gmail.com',
  password = crypto::argon2::generate('admin123'),
  status = 'active',
  user_type = 'super_admin',
  is_verified = true,
  is_active = true;

LET $user_id = (SELECT id FROM users WHERE email='emsanchez891@gmail.com')[0].id;

-- Crear el profile
CREATE profiles SET 
  user = $user_id,
  first_name = 'Elvin',
  last_name = 'Sanchez',
  birth_date = <datetime>'1994-06-30T00:00:00Z',
  phone = '+50499439493';

LET $profile_id = (SELECT id FROM profiles WHERE user=$user_id LIMIT 1)[0].id;

-- Actualizar el user con el profile
UPDATE $user_id SET profile = $profile_id;

-- Asignar roles al usuario
CREATE users_roles SET 
  user = $user_id,
  role = roles:super_admin;
