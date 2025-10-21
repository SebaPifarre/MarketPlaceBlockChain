# Trabajo Práctico Final – Marketplace Descentralizado tipo MercadoLibre (grupo 1)

Proyecto del grupo 1 para el seminario Rust 2025. [Enunciado](https://gist.github.com/EmaBord/980947ecc7924dd68c985e89c05916c2).

## Segunda entrega

### Marketplace

Address: pendiente de despliegue

Code hash: pendiente de despliegue

Contract hash: pendiente de despliegue

### ReportesView

Address: pendiente de despliegue

Code hash: pendiente de despliegue

Contract hash: pendiente de despliegue

## Primera entrega (Marketplace)

Address: XidV7PdUSfKaxX5V27StDY3MZG4x5TRcCMJaJF783kwsKrc

Code hash: 0xa400a27847949a65619ffb54686c904fcbf0a5b19dd361c5ef7166c4ea5b98cd

Contract hash: 0x6341fa7fc564c9d4ea6081b2d1ddf2aa4b96d13571e0cee6d4b2ce10194d96bc

## Estructura del contrato

### `Sistema`

La estructura principal de almacenamiento del contrato. Contiene todos los datos persistentes del marketplace:

-   `usuarios`: Un mapeo de `AccountId` a `Usuario`, almacenando la información de todos los usuarios registrados.
-   `publicaciones`: Un vector que contiene todas las publicaciones de productos activos en el sistema.
-   `productos`: Un mapeo de `u128` (ID de producto) a `Producto`, almacenando los detalles de cada producto.
-   `ordenes`: Un vector con todas las órdenes de compra generadas.
-   `proximo_id_publicacion`: Contador para generar IDs únicos para nuevas publicaciones.
-   `proximo_id_producto`: Lleva cuenta del último ID generado para productos.
-   `proximo_id_orden`: Lleva cuenta del último ID generado para órdenes de compra.

### `Usuario`

Representa a un usuario registrado en el sistema.

-   `nombre`: Nombre del usuario.
-   `apellido`: Apellido del usuario.
-   `email`: Correo electrónico del usuario.
-   `id`: `AccountId` único del usuario.
-   `rol`: Rol del usuario (`Comprador`, `Vendedor`, `Ambos`).
-   `publicaciones`: Vector de IDs de publicaciones (para usuarios `Vendedor`).
-   `ordenes`: Vector de IDs de órdenes de compra (para usuarios `Comprador`).

### `Rol`

Un `enum` que define los roles que un usuario puede tener:

-   `Comprador`
-   `Vendedor`
-   `Ambos`

### `Producto`

Representa un producto que puede ser publicado para la venta.

-   `nombre`: Nombre del producto.
-   `descripcion`: Descripción detallada del producto.
-   `categoria`: Categoría a la que pertenece el producto.

### `Categoría`

Un `enum` que clasifica los productos:

-   `Limpieza`
-   `Tecnología`
-   `Música`
-   `Ropa`
-   `Calzado`
-   `Otros`

### `Publicación`

Representa un artículo específico puesto a la venta en el marketplace.

-   `id_publicacion`: Identificador único de la publicación.
-   `id_producto`: ID del producto asociado a esta publicación.
-   `id_publicador`: `AccountId` del vendedor.
-   `precio`: Precio del producto en esta publicación.
-   `stock`: Cantidad disponible para la venta.
-   `activa`: Booleano que indica si la publicación está activa (reservado para la segunda entrega).

### `OrdenCompra`

Representa una orden de compra creada por un comprador.

-   `lista_productos`: Vector de tuplas `(id_producto, cantidad)` que componen la orden.
-   `id_orden_compra`: Identificador único de la orden.
-   `estado`: Estado actual de la orden (Pendiente, Enviado, Recibido, Cancelado).
-   `id_comprador`: `AccountId` del comprador.
-   `id_vendedor`: `AccountId` del vendedor.
-   `solicitud_cancelacion`: `Option<AccountId>` para registrar quién solicitó la cancelación.
-   `monto`: Monto total de la orden.

### `EstadoOrdenCompra`

Un `enum` que describe el ciclo de vida de una orden de compra:

-   `Pendiente`
-   `Enviado`
-   `Recibido`
-   `Cancelado`

## Funciones principales

### Marketplace

-   **`new()`**: Constructor del contrato.
-   **`es_vendedor()`**: Verifica si el `caller` tiene el rol de `Vendedor` o `Ambos`.
-   **`es_comprador()`**: Verifica si el `caller` tiene el rol de `Comprador` o `Ambos`.
-   **`registrar_usuario(nombre, apellido, email, rol)`**: Registra un nuevo usuario en el sistema.
-   **`agregar_rol(rol)`**: Permite a un usuario existente añadir un rol adicional (ej. de `Comprador` a `Ambos`).
-   **`nuevo_producto(nombre, descripcion, categoria)`**: Crea un nuevo producto. Solo accesible para vendedores.
-   **`crear_publicacion(id_producto, precio, stock)`**: Crea una nueva publicación para un producto existente. Solo accesible para vendedores.
-   **`generar_orden_compra(lista_publicaciones_con_cantidades, dinero_disponible)`**: Permite a un comprador crear una orden de compra.
-   **`marcar_orden_como_enviada(id_actual)`**: Marca una orden de compra como "Enviada". Solo accesible para el vendedor de la orden.
-   **`marcar_orden_como_recibida(id_actual)`**: Marca una orden de compra como "Recibida". Solo accesible para el comprador de la orden.
-   **`cancelar_orden(id_actual)`**: Permite a un comprador o vendedor solicitar la cancelación de una orden. La orden se cancela si y solo si ambos la solicitan.
-   **`get_publicaciones()`**: Devuelve una lista de todas las publicaciones activas en el sistema.
-   **`ver_mis_ordenes()`**: Devuelve una lista de las órdenes de compra asociadas al `caller`.

### ReportesView

- **`consultar_top_5_vendedores()`**: Devuelve hasta 5 usuarios (Vendedor/Ambos) ordenados por puntaje como vendedor. (Result<Vec<Usuario>, ErrorSistema> si la verificación remota falla)
- **`consultar_top_5_compradores()`**: Devuelve hasta 5 usuarios (Comprador/Ambos) ordenados por puntaje como comprador.
- **`ver_productos_mas_vendidos(categoria)`**: Devuelve Vec<(id_producto, ventas)> ordenado descendente (top 10).
- **`estadisticas_por_categoria()`**: Result<Vec<(Categoria, total_ventas: u32, promedio_puntuacion: u8)>, ErrorSistema> — ventas y promedio por categoría; puede devolver ErrorSistema en caso de overflow.

## Ejecución de tests
Con el comando **`cargo test --lib`**.

## Compilación
**`./build.sh`** para ambos contratos.

O compilar individualmente:
- **`./build.sh marketplace`**
- **`./build.sh reportes`**