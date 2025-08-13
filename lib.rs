#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod usuarios_sistema {
    use ink::prelude::{string::String};
    use ink::storage::Mapping;   
    use ink::prelude::vec::Vec;
    use ink::prelude::collections::BTreeSet;

    #[ink(storage)]

    /// # Esta es la estructura del sistema del MarketPlace.
    /// Estructura principal de almacenamiento del contrato marketplace.
    ///
    /// `Sistema` contiene toda la información persistente del contrato, incluyendo usuarios, productos,
    /// publicaciones, órdenes y los contadores para generar nuevos IDs únicos.
    ///
    /// # Campos
    /// - `usuarios`: Mapeo de AccountId a struct Usuario, representa todos los usuarios registrados.
    /// - `publicaciones`: Vector con todas las publicaciones activas en el sistema.
    /// - `productos`: Mapeo de id de producto a struct Producto, representa todos los productos creados.
    /// - `ordenes`: Vector con todas las órdenes de compra generadas.
    /// - `proximo_id_publicacion`: Contador para el próximo id único de publicación.
    /// - `proximo_id_producto`: Contador para el próximo id único de producto.
    /// - `proximo_id_orden`: Contador para el próximo id único de orden de compra.
    ///
    /// # Ejemplo de uso
    /// ```
    ///      let sistema = Sistema::new();
    ///      sistema.registrar_usuario("Juan".to_string(), "Perez".to_string(), "juan@email.com".to_string(), Rol::Comprador);
    /// ```
    pub struct Sistema {
        usuarios: ink::storage::Mapping<AccountId, Usuario>,
        publicaciones: Vec<Publicacion>,
        productos: Mapping<u128, Producto>,
        ordenes: Vec<OrdenCompra>,
        proximo_id_publicacion: u128,
        proximo_id_producto: u128,
        proximo_id_orden: u128,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone, PartialEq, Eq, Debug)]

    pub enum ErrorSistema {
        UsuarioYaRegistrado,
        UsuarioNoExiste,
        RolYaEnUso,
        // Producto
        ProductosLleno,
        // Publicación
        UsuarioNoEsVendedor,
        UsuarioNoEsComprador,
        ProductoInvalido,
        PublicacionesLleno,
        CompraSinItems,
        PublicacionNoValida,
        StockInsuficiente,
        VendedorDistinto,
        IdDeOrdenNoValida,
        PublicacionRepetida,
        NoPuedeComprarCero,
        NoPuedeComprarPublicacionPropia,
        OperacionNoValida,
        CancelacionYaSolicitada,
        DineroInsuficiente,
        FueraDeRango,
        OrdenCancelada,
    }

    /// # Esta es la estructura de un usuario.
    /// Representa un usuario del sistema de marketplace.
    /// Contiene información personal, rol, publicaciones y órdenes asociadas.
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone, PartialEq, Eq, Debug)]

    pub struct Usuario{
        /// Nombre del Usuario
        nombre:String,

        /// Apellido del Usuario
        apellido:String,

        /// Correo electrónico del Usuario
        email:String,

        /// Código identificador (AccountID) del usuario
        id:AccountId,

        /// Ocupación que tiene el Usuario en la página
        rol: Rol,

        //productos: Option<Producto>, //Si es vendedor tiene que tener una lista de sus productos.
        //orden_compra: Option<OrdenDeCompra>, //Si es comprador tiene que tener una orden de compra.

        /// Lista de Publicaciones (Id de publicaciones) que tiene un Usuario ´Vendedor´
        publicaciones: Vec<u128>,

        /// Lista de ´Ordenes de Compra´ (Id de Ordenes de compra) que tiene un Usuario ´Comprador´
        ordenes: Vec<u128>,
    }
    
    
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone, PartialEq, Eq, Debug)]

    pub enum Rol {
        Comprador,
        Vendedor,
        Ambos,
    }

    /// # Esta es la estructura de un Producto.
    /// Representa un producto en una publicación de marketplace.
    /// 
    /// Contiene el nombre, descripción y la categoría de éste.
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct Producto{

        nombre: String,
        
        descripcion: String,

        categoria: Categoria,
    }



    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub enum Categoria {
        Limpieza,
        Tecnologia,
        Musica,
        Ropa,
        Calzado,
        Otros,
    }

    // Publicación


    /// Representa una publicación en el marketplace.
    /// 
    /// Cada publicación está asociada a un producto específico y a un usuario vendedor.
    /// Contiene información relevante para la venta, como el precio, el stock disponible y el estado de la publicación.
    ///
    /// # Campos
    /// - `id_publicacion`: Identificador único de la publicación.
    /// - `id_producto`: Identificador del producto publicado.
    /// - `id_publicador`: AccountId del usuario que publica (vendedor).
    /// - `precio`: Precio del producto en la publicación.
    /// - `stock`: Cantidad disponible para la venta.
    /// - `activa`: Indica si la publicación está activa o no.
    ///
    /// # Ejemplo de uso
    /// ```
    ///      let publicacion = Publicacion {
    ///        id_publicacion: 1,
    ///        id_producto: 10,
    ///        id_publicador: AccountId::from([0x1; 32]),
    ///        precio: 1000,
    ///        stock: 5,
    ///        activa: true,
    ///      };
    /// ```
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug, Copy, Clone)]
    pub struct Publicacion {
        id_publicacion: u128,
        id_producto: u128,
        id_publicador: AccountId,
        precio: u32,
        stock: u32,
        activa: bool,
    }

    #[derive(Debug, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct OrdenCompra {
        

        lista_productos: Vec<(u128, u32)>,
        // El vec lo pense con un vec de tuplas, con el id del producto y la cantidad comprada.

        // Se me ocurre que dentro del usuario podemos tener un vec de órdenes de compra
        // y para acceder a una en especifica que se use el id de orden
        id_orden_compra: u128,

        estado: EstadoOrdenCompra,

        id_comprador: AccountId,

        id_vendedor: AccountId,

        solicitud_cancelacion: Option<AccountId>,

        monto:u32,

    }

    #[derive(Debug, Clone, PartialEq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub enum EstadoOrdenCompra {
        Pendiente,
        Enviado,
        Recibido,
        Cancelado,
    }

    impl Sistema {

        // # Sistema::new()
        /// Crea una nueva instancia del sistema, inicializando los campos de almacenamiento.
        /// No recibe parámetros.
        /// Retorna una instancia de Sistema.
        /// El new, inicializa todos los campos del sistema en un estado Default.
        /// # Ejemplo
        /// ```
        ///      let sistema = Sistema::new();
        /// ```
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {  usuarios: Mapping::new(), publicaciones: Vec::<Publicacion>::new(), productos: Mapping::new(), ordenes:Vec::new(), proximo_id_publicacion: 0, proximo_id_producto: 0 , proximo_id_orden: 0}
        }



        /// Verifica si el usuario que llama existe.
        /// Retorna `Ok(true)` si existe, o un error si no existe.
        fn _existe_usuario(&self, id: AccountId) -> Result<bool, ErrorSistema> {
            if self.usuarios.get(&id).is_some() {
                Ok(true)
            } else {
                Err(ErrorSistema::UsuarioNoExiste)
            }
        }


        /// !Es_Vendedor()
        /// Verifica si el usuario que llama es vendedor o tiene ambos roles.
        /// Retorna `Ok(true)` si es vendedor o ambos, `Ok(false)` si no lo es, o un error si no existe.
        ///
        /// # Ejemplo
        /// ```
        ///      let es_vendedor = sistema.es_vendedor();
        /// ```
        #[ink(message)]
        pub fn es_vendedor(&self) -> Result<bool, ErrorSistema> { 
            let id = self.env().caller();  
            self._es_vendedor(id)
        }

        fn _es_vendedor(&self, id: AccountId) -> Result<bool, ErrorSistema> {
            //Si existe el usuario
                //lo encuentro
                //y verifico si es vendedor o ambos.
            //Si no existe -> ErrorSistema::UsuarioNoExiste

            if (self._existe_usuario(id)).is_err() {
                return Err(ErrorSistema::UsuarioNoExiste);
            } else {
                //Busco al usuario y verifico su rol.
                let user = self.usuarios.get(&id);
                match user.unwrap().rol {
                    Rol::Vendedor | Rol::Ambos => Ok(true),
                    _ => Ok(false),
                }   
            }
        }


        /// !Es_Comprador()
        /// Verifica si el usuario que llama es comprador o tiene ambos roles.
        /// Retorna `Ok(true)` si es comprador o ambos, `Ok(false)` si no lo es, o un error si no existe.
        ///
        /// # Ejemplo
        /// ```
        ///      let es_comprador = sistema.es_comprador();
        /// ```
        #[ink(message)]
        pub fn es_comprador(&self) -> Result<bool, ErrorSistema> { 
            let id = self.env().caller(); 
            self._es_comprador(id)
        }

        fn _es_comprador(&self, id: AccountId) -> Result<bool, ErrorSistema> {
            //Si existe el usuario
                //lo encuentro
                //y verifico si es comprador o ambos.
            //Si no existe -> ErrorSistema::UsuarioNoExiste

            if (self._existe_usuario(id)).is_err() {
                return Err(ErrorSistema::UsuarioNoExiste);
            } else {
                //Busco al usuario y verifico su rol.
                let user = self.usuarios.get(&id);
                match user.unwrap().rol {
                    Rol::Comprador | Rol::Ambos => Ok(true),
                    _ => Ok(false),
                }   
            }
        }

        //Funciones asociadas a usuarios.

        /// !Registrar_Usuario() 
        /// Registra un nuevo usuario en el sistema con los datos proporcionados.
        /// El usuario queda asociado al AccountId del caller.
        /// Retorna `Ok(())` si el registro fue exitoso, o un error si ya existe.
        ///
        /// # Ejemplo
        /// ```
        ///      sistema.registrar_usuario("Juan".to_string(), "Perez".to_string(), "juan@email.com".to_string(), Rol::Comprador);
        /// ```
        #[ink(message)]
        pub fn registrar_usuario(&mut self, nombre:String, apellido:String, email:String, rol:Rol) -> Result<(), ErrorSistema> {
            let id = self.env().caller(); // Se obtiene el AccountId del usuario que llama a la función.

            self._registrar_usuario(nombre, apellido, email, rol, id)?;
            Ok(())
        }


        
        fn _registrar_usuario(&mut self, nombre:String, apellido:String, email:String, rol:Rol, id:AccountId) -> Result<(), ErrorSistema>{
            // Chequear que el usuario a registrar no exista en el sistema. (Sólo registrar usuarios nuevos).
            if self.usuarios.get(&id).is_some() { //Busca match en el mapping.
                return Err(ErrorSistema::UsuarioYaRegistrado);
            }                
            
            self.usuarios.insert(id, &Usuario {nombre, apellido, email, id, rol, publicaciones: Vec::<u128>::new(), ordenes: Vec::<u128>::new()});
            Ok(())
        }



        /// Agrega un rol adicional al usuario que llama.
        /// Retorna `Ok(())` si el rol fue agregado, o un error si ya lo tiene o no existe.
        ///
        /// # Ejemplo
        /// ```
        ///      sistema.agregar_rol(Rol::Vendedor);
        /// ```
        #[ink(message)]
        pub fn agregar_rol(&mut self, rol: Rol) -> Result<(), ErrorSistema> {
            let id = self.env().caller(); // Se obtiene el AccountId del usuario que llama a la función.

            self._agregar_rol(rol, id)
        }

        fn _agregar_rol(&mut self, rol: Rol, id: AccountId) -> Result<(), ErrorSistema> { 
            // Verifica si el usuario existe.
            if let Some(mut user) = self.usuarios.get(&id) {  
                user.agregar_rol(rol.clone())?; //Llama a la función del usuario que modifica su rol. (Lo delega)
                self.usuarios.insert(&id, &user); //Lo guardo modificado en el mapping.
                Ok(())
            } else {
                Err(ErrorSistema::UsuarioNoExiste)
            }
        }


        /// La función se fija si el id de un produto es menor al id próximo del producto a
        /// cargar, comprobando si éste ya fue cargdo o no.
        /// Retorna `true` si el proucto existe, o false si no existe.
        ///
        /// # Ejemplo
        /// ```
        ///      let id: u64;
        ///      sistema.existe_producto( id);
        /// ```
        fn existe_producto(&self, id: u128) -> bool {
            self.proximo_id_producto > id
        }

        fn generar_id_producto(&mut self) -> Result<u128, ErrorSistema> {
            let proximo = self.proximo_id_producto.clone();
            match self.proximo_id_producto.checked_add(1) {
                Some(val) => {
                    self.proximo_id_producto = val;
                    Ok(proximo)
                }
                None => Err(ErrorSistema::ProductosLleno)
            }
        }


        /// #nuevo_producto()
        /// Crea un nuevo producto asociado al usuario que llama (debe ser vendedor).
        /// Retorna el id del producto creado o un error si no es vendedor.
        ///
        /// # Ejemplo
        /// ```
        ///     let id_producto = sistema.nuevo_producto("Laptop".to_string(), "Laptop gamer".to_string(), Categoria::Tecnologia)?;
        /// ```
        #[ink(message)]
        pub fn nuevo_producto(&mut self, nombre: String, descripcion: String, categoria: Categoria) -> Result<u128, ErrorSistema> {
            //El usuario que genera el producto debe existir en el sistema, y ser vendedor.
            if let Err(e) = self._existe_usuario(self.env().caller()) {
                return Err(e);
            }

            if let Ok(false) = self.es_vendedor() {
                return Err(ErrorSistema::UsuarioNoEsVendedor);
            }

            let id_producto = self.generar_id_producto()?;

            self.productos.insert(id_producto, &Producto {
                nombre,
                descripcion,
                categoria
            });

            Ok(id_producto)
        }

        // Publicación
        fn generar_id_publicacion(&mut self) -> Result<u128, ErrorSistema> {
            let proximo = self.proximo_id_publicacion.clone();
            match self.proximo_id_publicacion.checked_add(1) {
                Some(val) => {
                    self.proximo_id_publicacion = val;
                    Ok(proximo)
                }
                None => Err(ErrorSistema::PublicacionesLleno)
            }
        }



        /// #crear_Publicacion()
        /// Crea una nueva publicación para un producto existente.
        /// El usuario debe ser vendedor y el producto debe existir.
        /// Retorna `Ok(())` si la publicación fue creada, o un error en caso contrario.
        ///
        /// # Ejemplo
        /// ```
        ///     sistema.crear_publicacion(0, 1000, 10);
        /// ```
        #[ink(message)]
        pub fn crear_publicacion(&mut self, id_producto: u128, precio: u32, stock: u32) -> Result<(), ErrorSistema> {
            self._crear_publicacion(id_producto, precio, stock)?;
            Ok(())
        }

        pub fn _crear_publicacion(&mut self, id_producto: u128, precio: u32, stock: u32) -> Result<(), ErrorSistema> {
            let usuario_id = self.env().caller(); // Se busca con el AccountId de la cuenta asociada.

            if let Ok(false) = self.es_vendedor() {
                return Err(ErrorSistema::UsuarioNoEsVendedor);
            }
            
            if !self.existe_producto(id_producto) {
                return Err(ErrorSistema::ProductoInvalido);
            }

            if stock == 0 {
                return Err(ErrorSistema::StockInsuficiente);
            }

            // Este unwrap se puede realizar sin problema porque la funcion es_vendedor() ya verifica si existe el usuario.
            let mut usuario = self.usuarios.get(&usuario_id).unwrap();

            // Agrego la publicación
            let id_publicacion = self.generar_id_publicacion()?;

            self.publicaciones.push(Publicacion {
                id_publicacion,
                id_producto,
                id_publicador: usuario_id,
                precio,
                stock,
                activa: true,
            });

            // Agrego la publicación a la lista de publicaciones del usuario
            

            usuario.publicaciones.push(id_publicacion);

            self.usuarios.insert(&usuario_id, &Usuario {
                nombre: usuario.nombre,
                apellido: usuario.apellido,
                email: usuario.email,
                id: usuario.id,
                rol: usuario.rol,
                publicaciones: usuario.publicaciones,
                ordenes: usuario.ordenes,
            });

            Ok(())
        }

        // Orden de compra


        /// Genera una nueva orden de compra para el usuario que llama.
        /// Recibe una lista de tuplas (id_publicacion, cantidad).
        /// Retorna la orden creada o un error si hay algún problema.
        ///
        /// # Ejemplo
        /// ```
        ///     let orden = sistema.generar_orden_compra(vec![(0, 2), (1, 1)])?;
        /// ```
        #[ink(message)]
        pub fn generar_orden_compra(&mut self, lista_publicaciones_con_cantidades:Vec<(u128, u32)>, dinero_disponible: u32)->Result<OrdenCompra, ErrorSistema>{
            let caller = self.env().caller();
            return self._generar_orden_compra(lista_publicaciones_con_cantidades, dinero_disponible, caller);
        }
        
        // Recibe un vector con las publicaciones y la cantidad de cada una para armar la orden.
        fn _generar_orden_compra(&mut self, lista_publicaciones_con_cantidades:Vec<(u128, u32)> , dinero_disponible:u32, caller:AccountId) -> Result<OrdenCompra, ErrorSistema>{
            // Chequeo si el usuario que está tratando de realizar la compra tiene el rol debido.
            
            //Si no existe el usuario se propaga el error:
            self.es_comprador()?;
            // // Verifico que el usuario sea comprador.
            // Si no es comprador, retorno un error.
            if let comprador = self.es_comprador()? {
                if !comprador {
                    return Err(ErrorSistema::UsuarioNoEsComprador);
                }
            }



            // Verifico que por lo menos exista una compra.
            if lista_publicaciones_con_cantidades.is_empty() {
                return Err(ErrorSistema::CompraSinItems);
            }

            // Busco el id del vendedor.
            let vendedor_actual:AccountId;
            if let Some(publi) = self.publicaciones.iter().find(|x|x.id_publicacion == lista_publicaciones_con_cantidades[0].0) {
                vendedor_actual = publi.id_publicador;
            }
            else {
                return Err(ErrorSistema::PublicacionNoValida)
            }

            //Si el usuario que creó la publicación trata de realizar una compra hay error.

            if vendedor_actual == caller {
                return Err(ErrorSistema::NoPuedeComprarPublicacionPropia);
            }

            
            self.validar_orden(lista_publicaciones_con_cantidades.clone(), vendedor_actual.clone())?;

            let monto_total = self.validar_precio(lista_publicaciones_con_cantidades.clone(), dinero_disponible)?;


            // Una vez pasadas todas las validaciones, actualizo el stock.

            let lista_compra = self.actualizar_stock_de_orden(lista_publicaciones_con_cantidades);


            let id_orden = self.generar_id_orden()?;
            

            // Creo la orden.

            let orden = OrdenCompra {
                id_comprador: caller,
                lista_productos: lista_compra,
                id_orden_compra: id_orden,
                estado: EstadoOrdenCompra::Pendiente,
                id_vendedor: vendedor_actual,
                solicitud_cancelacion: None,
                monto: monto_total,
            };
            
            // Agrego la orden al vector de órdenes.
            self.ordenes.push(orden.clone());
        
            // Agrego al vector de ambos usuarios.
            self.agregar_orden_usuario(caller, id_orden)?;
            self.agregar_orden_usuario(vendedor_actual, id_orden)?;


            Ok(orden.clone())
            
        }

        fn agregar_orden_usuario(&mut self, user_id:AccountId, id_orden:u128)->Result<(), ErrorSistema>{
            if let Some(mut user) = self.usuarios.get(&user_id){
                user.ordenes.push(id_orden);
                self.usuarios.insert(&user_id, &user);
                return Ok(())
            }
            else {
                return Err(ErrorSistema::UsuarioNoExiste);
            }

        }

        fn validar_orden(&self, lista_publicaciones_con_cantidades:Vec<(u128, u32)>, vendedor_actual:AccountId)->Result<(), ErrorSistema>{
            // Itero sobre la lista de publicaciones con cantidades y voy chequeando si la compra es válida(id de publicaciones válida y cant válida).

            let mut vistos = BTreeSet::new();
            
            for (id_publicacion_actual, cant_productos) in lista_publicaciones_con_cantidades {

                //Check de que no compre dos veces de la misma publicación
                if !vistos.insert(id_publicacion_actual) {
                    return Err(ErrorSistema::PublicacionRepetida)
                }

                // Check que se trate de comprar aunque sea un item de la publicación.
                if cant_productos==0 {
                    return Err(ErrorSistema::NoPuedeComprarCero)
                }

                if let Some(publicacion_actual) = self.publicaciones.iter().find(|x| x.id_publicacion == id_publicacion_actual){

                    // Veo que todas las publicaciones sean del mismo vendedor.
                    if publicacion_actual.id_publicador != vendedor_actual {
                        return Err(ErrorSistema::VendedorDistinto)
                    }

                    // Veo que la publicación tenga el stock necesario para la compra.
                    if !publicacion_actual.tiene_stock_suficiente(cant_productos) {
                        return Err(ErrorSistema::StockInsuficiente)
                    }
                }
                else {
                    return Err(ErrorSistema::PublicacionNoValida)
                }
            }
            Ok(())
        
        }

        fn validar_precio(&self, lista_publicaciones_con_cantidades:Vec<(u128, u32)>, dinero_disponible: u32)->Result<u32, ErrorSistema>{
            let mut monto_total:u32=0;
            for (id_publicacion, cant_productos) in lista_publicaciones_con_cantidades {
                if let Some(publicacion_actual) = self.publicaciones.get(id_publicacion as usize){

                    let monto_actual = match publicacion_actual.precio.checked_mul(cant_productos) {
                        Some(val) => val,
                        None => return Err(ErrorSistema::FueraDeRango),
                    };
                    monto_total = match monto_total.checked_add(monto_actual) {
                        Some(val) => val,
                        None => return Err(ErrorSistema::FueraDeRango),
                    }
                }
                else {
                    return Err(ErrorSistema::PublicacionNoValida);
                }
            }

            if dinero_disponible >= monto_total {
                return Ok(monto_total)
            }
            else {
                return Err(ErrorSistema::DineroInsuficiente);
            }
        }

        fn generar_id_orden(&mut self)->Result<u128, ErrorSistema>{
            let proximo = self.proximo_id_orden.clone();
            match self.proximo_id_orden.checked_add(1){
                Some(val) => {
                    self.proximo_id_orden = val;
                    Ok(proximo)
                }
                None => Err(ErrorSistema::PublicacionesLleno)
            }
        }

        fn actualizar_stock_de_orden(&mut self, lista_publicaciones_con_cantidades:Vec<(u128, u32)>)->Vec<(u128,u32)>{
            let mut lista_productos = Vec::new();
            for(id_publi, cant_productos) in lista_publicaciones_con_cantidades{
                if let Some(posicion) = self.publicaciones.iter().position(|x| x.id_publicacion == id_publi){

                    if let Some(publicacion_actual) = self.publicaciones.get_mut(posicion) {
                        publicacion_actual.actualizar_stock(cant_productos);
                        lista_productos.push((publicacion_actual.id_producto, cant_productos));
                    }
                }
            }
            lista_productos
        }




        /// Marca una orden como enviada. Solo el vendedor puede hacerlo.
        /// Retorna `Ok(())` si la operación fue exitosa, o un error si no corresponde.
        ///
        /// # Ejemplo
        /// ```
        ///      sistema.marcar_orden_como_enviada(0);
        /// ```
        #[ink(message)]
        pub fn marcar_orden_como_enviada(&mut self, id_actual:u128)->Result<(), ErrorSistema> {
            let caller = self.env().caller();
            self._marcar_orden_como_enviada(id_actual, caller)
        }

        fn _marcar_orden_como_enviada(&mut self, id_actual:u128, caller:AccountId)->Result<(), ErrorSistema>{


            if let Some(orden_acutal) = self.ordenes.get_mut(id_actual as usize){
                if orden_acutal.id_vendedor != caller {
                    return Err(ErrorSistema::OperacionNoValida)
                } 
                match &orden_acutal.estado {
                    EstadoOrdenCompra::Pendiente => Ok(orden_acutal.estado = EstadoOrdenCompra::Enviado),
                    _ => return Err(ErrorSistema::OperacionNoValida),
                }
                 
            }
            else {
                return Err(ErrorSistema::IdDeOrdenNoValida);
            }
            
        }



        /// Marca una orden como recibida. Solo el comprador puede hacerlo.
        /// Retorna `Ok(())` si la operación fue exitosa, o un error si no corresponde.
        ///
        /// # Ejemplo
        /// ```
        ///      sistema.marcar_orden_como_recibida(0);
        /// ```
        #[ink(message)]
        pub fn marcar_orden_como_recibida(&mut self, id_actual:u128)->Result<(), ErrorSistema> {
            let caller = self.env().caller();
            self._marcar_orden_como_recibida(id_actual, caller)
        }

        fn _marcar_orden_como_recibida(&mut self, id_actual:u128, caller:AccountId)->Result<(), ErrorSistema>{
            

            if let Some(orden_acutal) = self.ordenes.get_mut(id_actual as usize){
                if orden_acutal.id_comprador != caller {
                    return Err(ErrorSistema::OperacionNoValida)
                } 
                match &orden_acutal.estado {
                    EstadoOrdenCompra::Enviado => Ok(orden_acutal.estado = EstadoOrdenCompra::Recibido),
                    _ => return Err(ErrorSistema::OperacionNoValida),
                }
                 
            }
            else {
                return Err(ErrorSistema::IdDeOrdenNoValida);
            }
            
        }




        /// Solicita la cancelación de una orden. Puede ser solicitada por comprador o vendedor.
        /// Si ambos la solicitan, la orden se cancela.
        /// Retorna `Ok(())` si la operación fue exitosa, o un error si no corresponde.
        ///
        /// # Ejemplo
        /// ```
        ///      sistema.cancelar_orden(0);
        /// ```
        #[ink(message)]
        pub fn cancelar_orden(&mut self, id_actual:u128)->Result<(), ErrorSistema> {
            let caller = self.env().caller();
            self._cancelar_orden(id_actual, caller)
        }

        fn _cancelar_orden(&mut self, id_actual:u128, caller:AccountId) -> Result<(), ErrorSistema> {
            

            if let Some(orden_actual) = self.ordenes.get_mut(id_actual as usize) {

                if orden_actual.estado == EstadoOrdenCompra::Cancelado {
                    return Err(ErrorSistema::OrdenCancelada);
                }

                if orden_actual.estado == EstadoOrdenCompra::Recibido {
                    return Err(ErrorSistema::OperacionNoValida);
                }

                if let Some(id_anterior) = orden_actual.solicitud_cancelacion {
                    if id_anterior == caller {
                        return Err(ErrorSistema::CancelacionYaSolicitada);
                    }
                    else {
                        if id_anterior == orden_actual.id_comprador || id_anterior == orden_actual.id_vendedor{
                            self.ordenes.get_mut(id_actual as usize).unwrap().estado = EstadoOrdenCompra::Cancelado;
                            return Ok(())
                        }
                    }
                }
                self.ordenes.get_mut(id_actual as usize).unwrap().solicitud_cancelacion = Some(caller);
                return Ok(())
                    
            }
            else {
                return Err(ErrorSistema::IdDeOrdenNoValida);
            }
            
        }




        /// Devuelve la lista de todas las publicaciones existentes en el sistema.
        ///
        /// # Ejemplo
        /// ```
        ///      let publicaciones = sistema.get_publicaciones();
        /// ```
        #[ink(message)]
        pub fn get_publicaciones(&self)->Vec<Publicacion>{
            self.publicaciones.clone()
        }

        /// Devuelve la lista de todas las publicaciones existentes en el sistema del vendedor que la llama.
        ///
        /// # Ejemplo
        /// ```
        ///      AGREGAR!!!! 
        /// ```
        #[ink(message)]
        pub fn get_publicaciones_propias(&self)-> Result<Vec<Publicacion>, ErrorSistema>{
            let caller = self.env().caller();
            self._get_publicaciones_propias(caller)
        }

        fn _get_publicaciones_propias(&self, caller:AccountId)-> Result<Vec<Publicacion>, ErrorSistema> {
            let mut publicaciones_propias = Vec::<Publicacion>::new();
            // Verifico si el usuario existe.

            if let Err(e) = self._existe_usuario(caller) {
                return Err(ErrorSistema::UsuarioNoExiste); // Si no existe, retorno un vector vacío.
            } else {
                if !self.es_vendedor().unwrap_or(false) {
                    return Err(ErrorSistema::UsuarioNoEsVendedor); // Si no es vendedor, retorno un vector vacío.
                }
            }

            // Si el usuario existe y es vendedor, busco sus publicaciones.
            // Itero sobre las publicaciones del usuario y las agrego al vector de publicaciones propias.
            // Si el usuario no tiene publicaciones, el vector quedará vacío.
            let mut publicaciones_propias = Vec::new();
            for publicacion in self.publicaciones.iter() {
                if publicacion.id_publicador == caller {
                    publicaciones_propias.push(publicacion.clone());
                }
            }

            Ok(publicaciones_propias)
        }





        /// Devuelve la lista de órdenes asociadas al usuario que llama.
        ///
        /// # Ejemplo
        /// ```
        ///   let mis_ordenes = sistema.ver_mis_ordenes();
        /// ```
        #[ink(message)]
        pub fn ver_mis_ordenes(&self)->Vec<OrdenCompra>{
            let caller = self.env().caller();
            self._ver_mis_ordenes(caller)
        }

        fn _ver_mis_ordenes(&self, caller:AccountId)->Vec<OrdenCompra>{
            let mut mis_ordenes = Vec::new();
            if let Some(user) = self.usuarios.get(caller){
                for id in user.ordenes {
                    if let Some(orden) = self.ordenes.get(id as usize){
                        mis_ordenes.push(orden.clone())
                    }
                    
                }
            }
            mis_ordenes
        }
    }

    impl Usuario {
        pub fn agregar_rol(&mut self, rol: Rol) -> Result<(), ErrorSistema> { 
            if self.rol == rol || self.rol == Rol::Ambos{
                return Err(ErrorSistema::RolYaEnUso);
            }
            // Agrega el nuevo rol al usuario.
            self.rol = match (self.rol.clone(), rol.clone()) {
                (Rol::Comprador, Rol::Vendedor) | (Rol::Vendedor, Rol::Comprador) => Rol::Ambos,
                _ => rol,
            };
            Ok(())
        }
    }

    impl Publicacion {
            fn actualizar_stock(&mut self, cant:u32)->Result<(),ErrorSistema>{
                match self.stock.checked_sub(cant){
                    Some(val) => {
                        self.stock = val;
                        Ok(())
                    }
                    None => Err(ErrorSistema::PublicacionesLleno)
                }
            }

            fn tiene_stock_suficiente(&self, cant:u32)->bool{
                self.stock >= cant
            }
        }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;


        /// We test that we can register a user.
        /// In this test the user is added successfully.
        
        
        //---------------------------------------------------------------------------------
        //TESTS REGISTRAR USUARIO:
        #[ink::test]
        fn registrar_usuario_comprador_okay() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();

            assert!(sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador).is_ok());

            //Chequeamos que el usuario se haya registrado correctamente.
            let usuario = sistema.usuarios.get(&alice);
            assert!(usuario.is_some());
        }

        #[ink::test]
        fn registrar_usuario_vendedor_okay() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();

            assert!(sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Vendedor).is_ok());

            //Chequeamos que el usuario se haya registrado correctamente.
            let usuario = sistema.usuarios.get(&alice);
            assert!(usuario.is_some());
        }

        #[ink::test]
        fn registrar_usuario_ambos_okay() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();

            assert!(sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos).is_ok());

            //Chequeamos que el usuario se haya registrado correctamente.
            let usuario = sistema.usuarios.get(&alice);
            assert!(usuario.is_some());
        }

         /// We test that we cannot register a user that already exists.
         #[ink::test]
         fn registrar_usuario_not_okay() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
 
            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);
 
            assert!(sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador).is_err());

            //Chequeamos que el usuario no se haya registrado nuevamente.
            assert!(sistema.usuarios.get(&alice).is_some());
         }

        //-------------------------------------------------------------------------------------
        //TESTS PRODUCTOS:

        #[ink::test]
        fn nuevo_producto_usuario_inexistente() {
            //Se testea que un usuario que no existe en la plataforma no pueda crear un producto.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            let mut sistema = Sistema::new();

            assert!(sistema.nuevo_producto(String::from("Laptop"), String::from("Laptop gamer"), Categoria::Tecnologia).is_err());
            // El usuario no existe, por lo tanto no puede crear un producto.

            //Chequear el estado posterior del sistema (no debería haber ningún producto).
            assert!(sistema.productos.get(0).is_none());
        }

        #[ink::test]
        fn test_nuevo_producto_error() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Comprador);

            let error = sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros).unwrap_err();
            assert_eq!(error, ErrorSistema::UsuarioNoEsVendedor);//Chequear el estado posterior del sistema (no debería haber ningún producto).
            assert!(sistema.productos.get(0).is_none());

            //Chequear el estado posterior del sistema (no debería haber ningún producto).
            assert!(sistema.productos.get(0).is_none());
        }

        #[ink::test]
        //Test en el que se registra un producto correctamente desde un usuario que es vendedor.
        fn test_nuevo_producto_usuario_vendedor() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            let id_producto = sistema.nuevo_producto(String::from("Laptop"), String::from("Laptop gamer"), Categoria::Tecnologia).unwrap();
            // Verifico que el producto se haya registrado correctamente.
            let producto = sistema.productos.get(&id_producto);
            assert!(producto.is_some());
        }

        #[ink::test]
        //Test en el que se registra un producto correctamente desde un usuario con ambos roles.
        fn test_nuevo_producto_usuario_ambos() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            let id_producto = sistema.nuevo_producto(String::from("Laptop"), String::from("Laptop gamer"), Categoria::Tecnologia).unwrap();
            // Verifico que el producto se haya registrado correctamente.
            let producto = sistema.productos.get(&id_producto);
            assert!(producto.is_some());
        }

       //-------------------------------------------------------------------------------------
       //TESTS FUNCIONES INTERNAS:

        #[ink::test]
        fn test_existe_usuario() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            assert!(sistema._existe_usuario(alice).is_ok());
        }

        #[ink::test]
        fn test_no_existe_usuaro() {
            let mut sistema = Sistema::new();

            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            //Pruebo con un usuario (bob) que no esté en el sistema.
            assert!(sistema._existe_usuario(bob).is_err());
        }

        #[ink::test]
        //Registro un usuario en el sistema, que es vendedor y verifico que exista (con ese rol).
        fn test_es_vendedor() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Vendedor);

            //Pruebo con un usuario (alice) que esté en el sistema y sea vendedor.
            assert!(matches!(sistema.es_vendedor(), Ok(true)));
        }

        #[ink::test]
        //Registro un usuario en el sistema, que no es vendedor y verifico exista sin ese rol (que el modulo es_vendedor retorne falso).
        fn test_no_es_vendedor() {
            let mut sistema = Sistema::new();

            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Comprador);

            //Pruebo con un usuario (charlie) que esté en el sistema pero no sea vendedor.
            assert!(matches!(sistema.es_vendedor(), Ok(false)));
        }

        #[ink::test]
        //No registro a un usuario en el sistema, y verifico que no exista (que el modulo es_vendedor retorne error).
        fn test_es_vendedor_usuario_inexistente() {
            let mut sistema = Sistema::new(); 
            
            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            //Pruebo con un usuario (bob) que no esté en el sistema.
            assert!(sistema.es_vendedor().is_err());
        }


        #[ink::test]
        //Registro un usuario en el sistema, que es comprador y verifico que exista (con ese rol).
        fn test_es_comprador() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            //Pruebo con un usuario (alice) que esté en el sistema y sea comprador.
            assert!(matches!(sistema.es_comprador(), Ok(true)));
        }

        #[ink::test]
        //No registro a un usuario en el sistema, y verifico que no exista (que el modulo es_comprador retorne error).
        fn test_es_comprador_usuario_inexistente() {
            let mut sistema = Sistema::new(); 

            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            //Pruebo con un usuario (bob) que no esté en el sistema.
            assert!(sistema.es_comprador().is_err());
        }

        #[ink::test]
        //Registro un usuario en el sistema, que no es comprador y verifico exista sin ese rol (que el modulo es_comprador retorne falso).
        fn test_no_es_comprador() {
            let mut sistema = Sistema::new();

            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            //Pruebo con un usuario (charlie) que esté en el sistema pero no sea vendedor.
            assert!(matches!(sistema.es_comprador(), Ok(false)));
        }

        //-------------------------------------------------------------------------------------
        //TESTS AGREGAR_ROL:
        #[ink::test]
        //Se testea que se pueda agregar el rol de vendedor a un usuario que es comprador.
        fn test_agregar_roles_a_vendedor() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();
            //Inicializa alice como comprador.
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            //Se agrega el rol de vendedor (pasa a tener ambos).
            assert!(sistema.agregar_rol(Rol::Vendedor).is_ok());
            if let Some(user) = sistema.usuarios.get(&alice) {
                assert!(user.rol == Rol::Ambos);
            }
        }
        #[ink::test]
        //Se testea que se pueda agregar el rol de comprador a un usuario que es vendedor.
        fn test_agregar_roles_a_comprador() {
            //Inicializa bob como vendedor.
            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Bob"), String::from("Surname"), String::from("bob.email"), Rol::Vendedor);

            //Se agrega el rol de vendedor (pasa a tener ambos).
            assert!(sistema.agregar_rol(Rol::Comprador).is_ok());
            if let Some(user) = sistema.usuarios.get(&bob) {
                assert!(user.rol == Rol::Ambos);
            }
        }

        #[ink::test]
        //Se testea que se pueda agregar el rol de ambos a un usuario que es vendedor.
        fn test_agregar_roles_a_ambos() {
            //Inicializa bob como vendedor.
            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Bob"), String::from("Surname"), String::from("bob.email"), Rol::Vendedor);

            //Se agrega el rol de vendedor (pasa a tener ambos).
            assert!(sistema.agregar_rol(Rol::Ambos).is_ok());
            if let Some(user) = sistema.usuarios.get(&bob) {
                assert!(user.rol == Rol::Ambos);
            }
        }
        
        #[ink::test]
        //Se testea que no se pueda agregar un rol que ya tiene el usuario.
        fn test_agregar_roles_no_okay() {
            //Inicializa charlie como vendedor.
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            //Ya tiene el rol de vendedor. Por lo que no se puede agregar el rol de vendedor otra vez.
            let error = sistema.agregar_rol(Rol::Vendedor).unwrap_err();
            assert_eq!(error, ErrorSistema::RolYaEnUso);
        }

        #[ink::test]
        //Se testea que no se pueda agregar un rol a un usuario que no existe en el sistema.
        fn test_agregar_roles_usuario_inexistente() {
            let mut sistema = Sistema::new();
            let eve = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().eve;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(eve);

            //Pruebo con un usuario (eve) que no esté en el sistema.
            let error = sistema.agregar_rol(Rol::Vendedor).unwrap_err();
            assert_eq!(error, ErrorSistema::UsuarioNoExiste);
        }

        #[ink::test]
        /*TEST PARA EL FIX DE ESTA CORRECCIÓN: 
        "Existe una falla en la lógica que hace posible eliminarse roles al usar la función agregarRol() 
        teniendo ya el rol de Ambos. Permitiendo, por ejemplo, cambiar del rol Comprador a Ambos,
         para posteriormente pasar a tener únicamente el rol Vendedor."  */
        fn agregar_rol_desde_ambos() {
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            // Agrego el rol de Comprador, debería seguir siendo Ambos.
            assert!(sistema.agregar_rol(Rol::Comprador).is_err());
            let error = sistema.agregar_rol(Rol::Comprador).unwrap_err();
            assert_eq!(error, ErrorSistema::RolYaEnUso);

            if let Some(user) = sistema.usuarios.get(&charlie) {
                assert!(user.rol == Rol::Ambos);
            }
        }

        //-------------------------------------------------------------------------------------
        //TESTS ORDEN DE COMPRA:

        #[ink::test]
        //Test para verificar que no se puede generar una orden de compra sin items.
        fn generar_orden_compra_sin_items() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            //Pruebo generar una orden de compra sin items.
            let error = sistema.generar_orden_compra(Vec::<(u128, u32)>::new(), 100).unwrap_err();
            assert_eq!(error, ErrorSistema::CompraSinItems);
        }

        #[ink::test]
        //Test para verificar que no se puede generar una orden de compra de una publicacion que no existe.
        fn test_generar_orden_compra_error() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            //Quiero forzar el error de publicacionNoValida
            //No existe la publicación con id 0.
            let error_publicacion_invalida = sistema.generar_orden_compra(vec![(0, 1)],1).unwrap_err();
            assert_eq!(error_publicacion_invalida, ErrorSistema::PublicacionNoValida); //Ok

            //Verifico que no se haya agregado ninguna orden de compra. (Estado posterior del sistema).
            assert!(sistema.ordenes.is_empty());
        }

        #[ink::test]
        //Test para verificar que no se puede generar una orden de compra de una publicación propia.
        fn test_generar_orden_compra_propia() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            //Quiero forzar el error de NoPuedeComprarPublicacionPropia
            //Charlie crea una publicación y luego intenta comprarla.
            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4);

            let error_no_puede_comprar_publicacion_propia = sistema.generar_orden_compra(vec![(0, 1)],4000).unwrap_err();
            assert_eq!(error_no_puede_comprar_publicacion_propia, ErrorSistema::NoPuedeComprarPublicacionPropia); //Ok.

            //Verifico que no se haya agregado ninguna orden de compra. (Estado posterior del sistema).
            assert!(sistema.ordenes.is_empty());
        }

        #[ink::test]
        //Test para verificar que no se puede generar una orden de compra con dinero insuficiente.
        fn test_generar_orden_compra_dinero_insuficiente() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Preparo al otro usuario para que compre de esa publicación. (Ya que no se puede generar una orden de compra a partir de una publicación propia).
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);
            //Alice intenta comprar 1 termo, pero no tiene suficiente dinero (solo tiene 500).

            //Quiero forzar el error de DineroInsuficiente
            let error_dinero_insuficiente = sistema.generar_orden_compra(vec![(0, 1)], 500).unwrap_err();
            assert_eq!(error_dinero_insuficiente, ErrorSistema::DineroInsuficiente); //Ok.

            //Verifico que no se haya agregado ninguna orden de compra. (Estado posterior del sistema).
            assert!(sistema.ordenes.is_empty());
        }

        #[ink::test]
        //Test para verificar que un usuario no pueda generar una orden de compra si no es comprador.
        fn test_orden_compra_no_comprador() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            //Quiero forzar el error de UsuarioNoEsComprador
            let error_usuario_no_comprador = sistema.generar_orden_compra(vec![(0, 1)], 1000).unwrap_err();
            assert_eq!(error_usuario_no_comprador, ErrorSistema::UsuarioNoEsComprador); //Ok.

            //Verifico que no se haya agregado ninguna orden de compra. (Estado posterior del sistema).
            assert!(sistema.ordenes.is_empty());
        }

        #[ink::test]
        //Test para verificar que un usuario no pueda generar una orden de compra si no existe en el sistema.
        fn test_orden_compra_usuario_inexistente() {
            let mut sistema = Sistema::new();
            let eve = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().eve;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(eve);

            //Quiero forzar el error de UsuarioNoExiste
            let error_usuario_no_existe = sistema.generar_orden_compra(vec![(0, 1)], 1000).unwrap_err();
            assert_eq!(error_usuario_no_existe, ErrorSistema::UsuarioNoExiste); //Ok.

            //Verifico que no se haya agregado ninguna orden de compra. (Estado posterior del sistema).
            assert!(sistema.ordenes.is_empty());
        }


        #[ink::test]
        //Test para verificar que se pueda generar una orden de compra correctamente.
        fn test_ver_mis_ordenes() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Preparo al otro usuario para que compre de esa publicación. (Ya que no se puede generar una orden de compra a partir de una publicación propia).
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);
            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());

            //Verifico que la orden de compra se haya agregado a las órdenes del usuario Alice.
            let mis_ordenes = sistema.ver_mis_ordenes();
            assert_eq!(mis_ordenes.len(), 1);
        }

        #[ink::test]
        fn test_agregar_orden_usuario() {
            let mut sistema = Sistema::new();

            let error_usuario_no_existe = sistema.agregar_orden_usuario(AccountId::from([0x1; 32]), 1).unwrap_err();
            assert_eq!(error_usuario_no_existe, ErrorSistema::UsuarioNoExiste); //No existe el usuario que llama a la función.
        }

        #[ink::test]
        //Test para verificar que no se puede agregar una orden de compra con publicaciones repetidas (no pasa la validación).
        fn test_validar_orden_publicacion_repetida() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de PublicacionRepetida.
            let error_publicacion_repetida = sistema.validar_orden(vec![(0, 1), (0, 2)], charlie).unwrap_err(); 
            assert_eq!(error_publicacion_repetida, ErrorSistema::PublicacionRepetida); 
        }

        #[ink::test]
        //Test para verificar que no se puede generar una orden de compra con cantidad cero (no pasa la validación).
        fn test_validar_orden_comprar_cero() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de NoPuedeComprarCero.
            let error_no_puede_comprar_cero = sistema.validar_orden(vec![(0, 0)], charlie).unwrap_err(); 
            assert_eq!(error_no_puede_comprar_cero, ErrorSistema::NoPuedeComprarCero); //Ok.
        }

        #[ink::test]
        //Test para verificar que no se puede generar una orden de compra con productos de distintos usuarios vendedores (no pasa la validación).
        fn test_validar_orden_vendedor_distinto() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de VendedorDistinto.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let error_vendedor_distinto = sistema.validar_orden(vec![(0, 1)], alice).unwrap_err();
            assert_eq!(error_vendedor_distinto, ErrorSistema::VendedorDistinto); //Ok.
        }

        #[ink::test]
        //Test para verificar que no se puede generar una orden de compra con stock insuficiente (no pasa la validación).
        fn test_validar_orden_compra_stock_insuficiente() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de StockInsuficiente.
            let error_stock_insuficiente = sistema.validar_orden(vec![(0, 5)], charlie).unwrap_err(); //El stock es 4, y estoy tratando de comprar 5.
            assert_eq!(error_stock_insuficiente, ErrorSistema::StockInsuficiente); //Ok.
        }

        #[ink::test]
        //Test para verificar que no se puede agregar una orden de compra con publicaciones no válidas (no pasa la validación).
        fn test_validar_orden_publicacion_no_valida() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de PublicacionNoValida.
            let error_publicacion_invalida = sistema.validar_orden(vec![(1, 1)], charlie).unwrap_err();
            assert_eq! (error_publicacion_invalida, ErrorSistema::PublicacionNoValida); //Ok.
        }

        //-------------------------------------------------------------------------------------
        //TEST ESTADOS DE ORDEN
  
        #[ink::test]
        //Este test es para ver si salta el error (operación no válida) al tratar de cancelar una orden ya recibida (ya que es algo que no se puede hacer).
        fn cancelar_orden_ya_recibida() {
            //Genero una orden de compra
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            //Creo el producto.
            if let Ok(id) = sistema.nuevo_producto("banana".to_string(), "una banana".to_string(), Categoria::Limpieza){
                assert_eq!(id, 0);
            }

            //Creo la publicación.
            sistema.crear_publicacion(0, 10, 19);

            let mut lista_compra = Vec::new();
            lista_compra.push((0,2));

            //Preparo al otro usuario para que compre de esa publicación. (Ya que no se puede generar una orden de compra a partir de una publicación propia).
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);
            
            assert!(sistema.generar_orden_compra(lista_compra, 200).is_ok());

            //Marco como enviado (desde Charlie).
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);

            assert!(sistema.marcar_orden_como_enviada(0).is_ok());
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Enviado);
            }

            //Marco como recibido (desde Alice).
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            assert!(sistema.marcar_orden_como_recibida(0).is_ok());

            //Trato de cancelar la orden (desde Alice) (esto debería fallar).
            let error = sistema.cancelar_orden(0).unwrap_err();
            assert_eq!(error, ErrorSistema::OperacionNoValida);


            //Trato de cancelar la orden también desde Charlie.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            let error = sistema.cancelar_orden(0).unwrap_err();
            assert_eq!(error, ErrorSistema::OperacionNoValida);


            //Chequeo estado del sistema posteriormente (ver si no se modificó el estado de la orden).
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Recibido);
            }
        }

        #[ink::test]
        //Test que verifica que no se puede marcar la orden como enviada si el id de la misma es inválido.
        fn test_marcar_orden_como_enviada_id_invalido() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de IdDeOrdenNoValida.
            let error_id_invalido = sistema.marcar_orden_como_enviada(0).unwrap_err();
            assert_eq!(error_id_invalido, ErrorSistema::IdDeOrdenNoValida); //No existe la orden con id 0.

            //Chequeo estado del sistema posteriormente (ver si no se modificó el estado de la orden).
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Pendiente);
            }
        }

        #[ink::test]
        ///Test que verifica que no se puede marcar una orden como enviada si el caller no es el vendedor de la orden.
        fn test_marcar_orden_enviada_caller_invalido() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());

            //Quiero forzar el error de OperacionNoValida.
            let error_operacion_no_valida = sistema.marcar_orden_como_enviada(0).unwrap_err(); //La estoy tratando de marcar como enviada desde Alice, pero la orden la creó Charlie.
            assert_eq!(error_operacion_no_valida, ErrorSistema::OperacionNoValida); //El caller no es el vendedor de la orden.

            //Chequeo estado del sistema posteriormente (ver si no se modificó el estado de la orden).
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Pendiente);
            }
        }

        #[ink::test]
        //Test que verifica que no se puede marcar una orden como enviada si la orden ya fue enviada.
        fn test_marcar_orden_como_enviada_doble() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());

            //Quiero forzar el error de OperacionNoValida porque la orden ya fue enviada.

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);

            assert!(sistema.marcar_orden_como_enviada(0).is_ok()); //Primero lo marco como enviada.

            //Trato de enviarlo otra vez.
            let error_operacion_no_valida = sistema.marcar_orden_como_enviada(0).unwrap_err();
            assert_eq!(error_operacion_no_valida, ErrorSistema::OperacionNoValida); //La orden ya fue enviada.

            //Chequeo estado del sistema posteriormente (ver si el estado de la orden quedó como enviado).
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Enviado);
            }
        }

        #[ink::test]
        //Test para verificar que se puede marcar una orden como enviada correctamente.
        fn test_marcar_orden_enviada_okay() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            //Genero la orden de compra.
            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());


            //Quiero marcar la orden como recibida.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            assert!(sistema.marcar_orden_como_enviada(0).is_ok()); //Lo marco como enviada.

            //Chequeo el estado de la orden. (Estado posterior del sistema).
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Enviado);
            } else {
                panic!("La orden no fue encontrada después de marcarla como enviada.");
            }

        }

        #[ink::test]
        //Test que verifica que se puede marcar una orden como recibida correctamente.
        fn test_marcar_orden_como_recibida() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            //Genero la orden de compra.
            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());


            //Quiero marcar la orden como recibida.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.marcar_orden_como_enviada(0); //primero lo marco como enviada

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            assert_eq!(sistema.marcar_orden_como_recibida(0), Ok(()));

            //Chequeo el estado de la orden. (Estado posterior del sistema).
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Recibido);
            } else {
                panic!("La orden no fue encontrada después de marcarla como recibida.");
            }
        }

        #[ink::test]
        //Test que verifica que no se puede marcar una orden como recibida si quien lo hace es el usuario que la creó.
        fn test_marcar_orden_recibida_mismo_caller() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            //Genero la orden de compra.
            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());


            //Quiero marcar la orden como recibida.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.marcar_orden_como_enviada(0); //primero lo marco como enviada

            //Chequeo que el usuario que marcó como enviada no pueda marcar como recibida. (No cambié el caller).
            if let Err(e) = sistema.marcar_orden_como_recibida(0) {
                assert_eq!(e, ErrorSistema::OperacionNoValida);
            }

            //Chequeo el estado de la orden. (Estado posterior del sistema).
            //La orden no debería haber cambiado su estado.
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Enviado);
            }
        }

        #[ink::test]
        //Test que verifica que no se puede marcar una orden como recibida si el id de la misma es inválido.
        fn test_marcar_orden_enviada_orden_invalida() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de IdDeOrdenNoValida.
            let error_id_invalido = sistema.marcar_orden_como_enviada(0).unwrap_err();
            assert_eq!(error_id_invalido, ErrorSistema::IdDeOrdenNoValida); //No existe la orden con id 0.
        }

        #[ink::test]
        //Test que verifica que no se puede marcar una orden como enviada, que ya fue (previamente) recibida.
        fn test_marcar_orden_enviada_orden_recibida() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            //Genero la orden de compra.
            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());

            //Quiero marcar la orden como enviada.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            assert!(sistema.marcar_orden_como_enviada(0).is_ok()); //Lo marco como enviada.

            //Ahora quiero marcarla como recibida.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            assert!(sistema.marcar_orden_como_recibida(0).is_ok()); //Lo marco como recibida.

            //Quiero forzar el error de OperacionNoValida porque la orden ya fue recibida.
            let error_operacion_no_valida = sistema.marcar_orden_como_enviada(0).unwrap_err();
            assert_eq!(error_operacion_no_valida, ErrorSistema::OperacionNoValida); //La orden ya fue recibida.

            //Chequeo el estado de la orden. (Estado posterior del sistema).
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Recibido);
            } else {
                panic!("La orden no fue encontrada después de marcarla como recibida.");
            }
        }

        #[ink::test]
        //Test que verifica que no se puede marcar una orden como recibida si la orden no fue marcada como enviada previamente.
        fn test_marcar_orden_recibida_sin_envio() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());


            //Quiero forzar el error de OperacionNoValida.
            let error_operacion_no_valida = sistema.marcar_orden_como_recibida(0).unwrap_err();
            assert_eq!(error_operacion_no_valida, ErrorSistema::OperacionNoValida); //El caller trata de marcar la orden como recibida sin que esta fuera marcada como enviada previamente.
        }

        #[ink::test]
        //Test que verifica que no se puede marcar una orden como recibida si el caller no es el comprador de la orden.
        fn test_marcar_orden_recibida_caller_invalido() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());

            //Primero la marco como enviada desde quien creo la publicación (Charlie).
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.marcar_orden_como_enviada(0); //Primero lo marco como enviada.

            //Quiero forzar el error de OperacionNoValida.
            let error_caller_invalido = sistema.marcar_orden_como_recibida(0).unwrap_err();
            assert_eq!(error_caller_invalido, ErrorSistema::OperacionNoValida); //El caller no es el comprador de la orden.
        }        


        #[ink::test]
        //Test que verifica que no se puede cancelar una orden cuando el id de la misma es inválido.
        fn test_cancelar_orden_invalida() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de IdDeOrdenNoValida.
            let error_id_invalido = sistema.cancelar_orden(0).unwrap_err();
            assert_eq!(error_id_invalido, ErrorSistema::IdDeOrdenNoValida); //No existe la orden con id 0.     
        }

        #[ink::test]
        //Test que verifica que se puede cancelar una orden correctamente.
        fn test_cancelar_orden() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());

            //Quiero cancelar la orden.
            //Primero cancelo desde quien lo compró (alice).
            assert!(sistema.cancelar_orden(0).is_ok());

            //Chequeo  que el estado de la orden no se modificó todavía (porque falta la segunda parte de la cancelación).
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Pendiente);
            } else {
                panic!("La orden no fue encontrada después de cancelarla.");
            }

            //Ahora cancelo desde quien la creó (Charlie).
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            assert!(sistema.cancelar_orden(0).is_ok());

            //Chequeo que el estado de la orden cambió a cancelado.
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Cancelado);
            } else {
                panic!("La orden no fue encontrada después de cancelarla.");
            }
        }


        //-------------------------------------------------------------------------------------
        //TESTS PUBLICACIONES Y STOCK:

        #[ink::test]
        //Test para verificar que se puede crear una publicación correctamente.
        fn test_crear_publicacion() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            if let Ok(id) = sistema.nuevo_producto("banana".to_string(), "una banana".to_string(), Categoria::Limpieza){
                assert_eq!(id, 0);
            }


            //Chequeo el estado posterior del sistema (que se haya creado la publicación).
            sistema.crear_publicacion(0, 10, 19);
            assert_eq!(sistema.get_publicaciones().len(), 1);
        }

        #[ink::test]
        //Test para verificar que tiene_stock_suficiente funcione correctamente.
        fn test_publicacion_tiene_stock_suficiente(){
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            if let Ok(id) = sistema.nuevo_producto("banana".to_string(), "una banana".to_string(), Categoria::Limpieza){
                assert_eq!(id, 0);
            }

            sistema.crear_publicacion(0, 10, 19);
            assert_eq!(sistema.get_publicaciones().len(), 1);

            assert_eq!(sistema.publicaciones[0].tiene_stock_suficiente(5), true);
            assert_eq!(sistema.publicaciones[0].tiene_stock_suficiente(10), true);
        }

        #[ink::test]
        //Test que verifica que tiene_stock_suficiente funcione correctamente cuando el stock es insuficiente.
        fn test_publicacion_tiene_stock_insuficiente() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            if let Ok(id) = sistema.nuevo_producto("banana".to_string(), "una banana".to_string(), Categoria::Limpieza){
                assert_eq!(id, 0);
            }

            sistema.crear_publicacion(0, 10, 19); //Le doy 19 de stock.
            assert_eq!(sistema.get_publicaciones().len(), 1);

            assert_eq!(sistema.publicaciones[0].tiene_stock_suficiente(20), false);
        }


        #[ink::test]
        //Test que verifica que no se puede crear una publicación con stock 0.
        fn test_crear_publicacion_stock_cero() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            if let Ok(id) = sistema.nuevo_producto("banana".to_string(), "una banana".to_string(), Categoria::Limpieza){
                assert_eq!(id, 0);
            }

            //Intento crear una publicación con stock 0.
            let error_stock_cero = sistema.crear_publicacion(0, 10, 0).unwrap_err(); 
            assert_eq!(error_stock_cero, ErrorSistema::StockInsuficiente); //No se puede crear una publicación con stock 0.

            //Chequeo el estado posterior del sistema (que no se haya creado la publicación).
            assert_eq!(sistema.get_publicaciones().len(), 0);
        }

        #[ink::test]
        //Test para verificar que no se puede comprar de una publicación con dinero insuficiente.
        fn test_comprar_publicacion_dinero_insuficiente() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            if let Ok(id) = sistema.nuevo_producto("banana".to_string(), "una banana".to_string(), Categoria::Limpieza){
                assert_eq!(id, 0);
            }

            sistema.crear_publicacion(0, 10, 19); //Le doy 19 de stock. Cada banana sale 10 pesos.
            assert_eq!(sistema.get_publicaciones().len(), 1);

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let lista_compra = vec![(0, 1)];

            //Intento comprar una publicación con dinero insuficiente.
            let error_dinero_insuficiente = sistema.generar_orden_compra(lista_compra, 0).unwrap_err(); //Trato de comprar una banana con 0 dinero.
            assert_eq!(error_dinero_insuficiente, ErrorSistema::DineroInsuficiente); //No se puede comprar la publicación porque el dinero es insuficiente.

            //Chequeo el estado posterior del sistema (que no se haya modificado el stock).
            assert_eq!(sistema.publicaciones[0].stock, 19);
        }

        #[ink::test]
        //Test para verificar que un usuario que no es vendedor o ambos pueda crear una publicación.
        fn test_crear_publicacion_user_no_vendedor() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Comprador);

            let error_user_no_vendedor = sistema.crear_publicacion(0, 1000, 4).unwrap_err();
            assert_eq!(error_user_no_vendedor, ErrorSistema::UsuarioNoEsVendedor); //Ok.

            //Chequeo el estado posterior del sistema (no debe existir ninguna publicación).
            assert_eq!(sistema.get_publicaciones().len(), 0);
        }

        #[ink::test]
        //Test para verificar que no se puede crear una publicación de un producto inválido.
        fn test_crear_publicacion_producto_invalido() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            let error_producto_invalido = sistema.crear_publicacion(0, 1000, 4).unwrap_err();
            assert_eq!(error_producto_invalido, ErrorSistema::ProductoInvalido); //No existe el producto con id 0.

            //Chequeo el estado posterior del sistema (no debe existir ninguna publicación).
             assert_eq!(sistema.get_publicaciones().len(), 0);
        }

        #[ink::test]
        //Test que verifica que get publicaciones_propias funcione (con un caso en el que sí funciona).
        fn get_publicaciones_propias_okay() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Verifico que el usuario pueda obtener sus publicaciones.
            assert!(sistema.get_publicaciones_propias().is_ok());

            //Chequeo el estado posterior del sistema.
            assert_eq!(sistema.get_publicaciones().len(), 1);
        }

        #[ink::test]
        //Test que verifica que get publicaciones_propias funcione (con un caso en el que el usuario es existe, es vendedor y no tiene publicaciones).
        fn get_publicaciones_propias_vacio_okay() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            //Verifico que el usuario pueda obtener sus publicaciones (debe devolver un vector vacío).
            let publicaciones_propias = sistema.get_publicaciones_propias().unwrap();
            assert!(publicaciones_propias.is_empty());

            //Chequeo el estado posterior del sistema.
            assert_eq!(sistema.get_publicaciones().len(), 0);
        }

        #[ink::test]
        //Test que verifica que get publicaciones_propias 'no funcione' (con un caso en el que el usuario no existe en el sistema).
        fn get_publicaciones_propias_usuario_no_existe() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            //No registro a charlie en el sistema.

            let error_usuario_no_existe = sistema.get_publicaciones_propias().unwrap_err();
            assert_eq!(error_usuario_no_existe, ErrorSistema::UsuarioNoExiste); //El usuario no existe.
        }

        #[ink::test]
        //Test que verifica que get publicaciones_propias 'no funcione' (con un caso en el que el usuario no es vendedor).
        fn get_publicaciones_propias_usuario_no_vendedor() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Comprador);

            let error_usuario_no_es_vendedor = sistema.get_publicaciones_propias().unwrap_err();
            assert_eq!(error_usuario_no_es_vendedor, ErrorSistema::UsuarioNoEsVendedor); //El usuario no es vendedor.
        }


        //-------------------------------------------------------------------------------------
        //TESTS PRECIO Y CHECKED SUMS:

        #[ink::test]
        fn test_calculo_precio(){
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);
            sistema.nuevo_producto("Cif".to_string(), "Cif".to_string(), Categoria::Limpieza);
            sistema.nuevo_producto("Remera".to_string(), "Remera".to_string(), Categoria::Ropa);
            sistema.crear_publicacion(0, 10, 19);
            sistema.crear_publicacion(1, 20, 5);

            sistema.nuevo_producto("Precioalto".to_string(), "Precioalto".to_string(), Categoria::Ropa);
            let precio_alto = u32::MAX;
            sistema.crear_publicacion(2, precio_alto, 5);

            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let mut lista_compra = Vec::new();
            lista_compra.push((0,2));
            lista_compra.push((1,3));


            if let Err(e) = sistema.generar_orden_compra(lista_compra.clone(), 70){
                assert_eq!(e, ErrorSistema::DineroInsuficiente);
            }

            if let Ok(ord) = sistema.generar_orden_compra(lista_compra.clone(), 200){
                assert_eq!(ord.monto, 80);
            }

            if let Err(e) = sistema.generar_orden_compra(vec![(1,1), (2,1)], 200) {
                assert_eq!(e, ErrorSistema::FueraDeRango);
            }

            if let Err(e) = sistema.generar_orden_compra(vec![(2,3)], 200) {
                assert_eq!(e, ErrorSistema::FueraDeRango);
            }

            lista_compra.push((999,1));

            if let Err(e) = sistema.validar_precio(lista_compra.clone(), 200){
                assert_eq!(e, ErrorSistema::PublicacionNoValida);
            }

        }

        #[ink::test]
        fn test_checked_sums(){
            let mut sistema = Sistema::new();
            sistema.proximo_id_producto = u128::MAX;
            let result = sistema.generar_id_producto();
            assert_eq!(result.unwrap_err(), ErrorSistema::ProductosLleno);

            sistema.proximo_id_publicacion = u128::MAX;
            if let Err(e) = sistema.generar_id_publicacion() {
                assert_eq!(e, ErrorSistema::PublicacionesLleno);
            }

            sistema.proximo_id_orden = u128::MAX;
            if let Err(e) = sistema.generar_id_orden() {
                assert_eq!(e, ErrorSistema::PublicacionesLleno);
            }

            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);
            sistema.nuevo_producto("Cif".to_string(), "Cif".to_string(), Categoria::Limpieza);

            sistema.crear_publicacion(0, 10, 19);

            if let Some(p) = sistema.publicaciones.get_mut(0) {
                assert_eq!(p.actualizar_stock(u32::MAX), Err(ErrorSistema::PublicacionesLleno))
            }

        }

        //-------------------------------------------------------------------------------------
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = UsuariosRef::default();

            // When
            let contract = client
                .instantiate("usuarios", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Usuarios>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = UsuariosRef::new(false);
            let contract = client
                .instantiate("usuarios", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Usuarios>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
