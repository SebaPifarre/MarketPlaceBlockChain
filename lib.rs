#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod usuarios_sistema {
    use ink::prelude::{string::String};
    use ink::storage::Mapping;   
    use ink::prelude::vec::Vec;
    use ink::prelude::collections::BTreeSet;

    #[ink(storage)]

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
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
    }
   
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone, PartialEq, Eq, Debug)]

    pub struct Usuario{
        nombre:String,
        apellido:String,
        email:String,
        id:AccountId,
        rol: Rol,
        publicaciones: Vec<u128>,

        // Vector con la posicion en el vector del sistema 
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

    // Producto
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
        // El vec lo pense con un vec de tuplas, con el id del producto y la cantidad comprada.
        lista_productos: Vec<(u128, u32)>,

        // Se me ocurre que dentro del usuario podemos tener un vec de ordenes de compra
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
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {  usuarios: Mapping::new(), publicaciones: Vec::<Publicacion>::new(), productos: Mapping::new(), ordenes:Vec::new(), proximo_id_publicacion: 0, proximo_id_producto: 0 , proximo_id_orden: 0}
        }


        //Verificadores del sistema.
        fn _existe_usuario(&self, id: AccountId) -> Result<bool, ErrorSistema> {
            if self.usuarios.get(&id).is_some() {
                Ok(true)
            } else {
                Err(ErrorSistema::UsuarioNoExiste)
            }
        }

        #[ink(message)]
        pub fn es_vendedor(&self) -> Result<bool, ErrorSistema> { 
            let id = self.env().caller();  
            self._es_vendedor(id)
        }

        fn _es_vendedor(&self, id: AccountId) -> Result<bool, ErrorSistema> {
            //Si existe el usuario
                //lo encuentro
                //y verifico si es vendedor o ambos.
            //Si no existo -> ErrorSistema::UsuarioNoExiste

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

        #[ink(message)]
        pub fn es_comprador(&self) -> Result<bool, ErrorSistema> { 
            let id = self.env().caller(); 
            self._es_comprador(id)
        }

        fn _es_comprador(&self, id: AccountId) -> Result<bool, ErrorSistema> {
            //Si existe el usuario
                //lo encuentro
                //y verifico si es comprador o ambos.
            //Si no existo -> ErrorSistema::UsuarioNoExiste

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

        #[ink(message)]
        pub fn registrar_usuario(&mut self, nombre:String, apellido:String, email:String, rol:Rol) -> Result<(), ErrorSistema> {
            let id = self.env().caller(); // Se obtiene el AccountId del usuario que llama a la función.

            self._registrar_usuario(nombre, apellido, email, rol, id)?;
            Ok(())
        }


        
        fn _registrar_usuario(&mut self, nombre:String, apellido:String, email:String, rol:Rol, id:AccountId) -> Result<(), ErrorSistema>{
            // Chequear que el usuario a registrar no exista en el sistema. (Solo registrar usuarios nuevos)
            if self.usuarios.get(&id).is_some() { //Busca match en el mapping.
                return Err(ErrorSistema::UsuarioYaRegistrado);
            }                
            
            self.usuarios.insert(id, &Usuario {nombre, apellido, email, id, rol, publicaciones: Vec::<u128>::new(), ordenes: Vec::<u128>::new()});
            Ok(())
        }

        #[ink(message)]
        pub fn agregar_rol(&mut self, rol: Rol) -> Result<(), ErrorSistema> {
            let id = self.env().caller(); // Se obtiene el AccountId del usuario que llama a la función.

            self._agregar_rol(rol, id)
        }

        fn _agregar_rol(&mut self, rol: Rol, id: AccountId) -> Result<(), ErrorSistema> { 
            // Verifica si el usuario existe.
            if let Some(mut user) = self.usuarios.get(&id) {  
                user.agregar_rol(rol.clone())?; //Llama a la función del usuario que modifica su rol. (Lo delega)
                self.usuarios.insert(&id, &user); //Lo guardo modificado en le mapping.
                Ok(())
            } else {
                Err(ErrorSistema::UsuarioNoExiste)
            }
        }

        // Producto
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

        #[ink(message)]
        pub fn nuevo_producto(&mut self, nombre: String, descripcion: String, categoria: Categoria) -> Result<u128, ErrorSistema> {
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

            let mut usuario = match self.usuarios.get(&usuario_id) {
                Some(u) => u,
                None => return Err(ErrorSistema::UsuarioNoExiste),
            };

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
        #[ink(message)]
        pub fn generar_orden_compra(&mut self, lista_publicaciones_con_cantidades:Vec<(u128, u32)>, dinero_disponible: u32)->Result<OrdenCompra, ErrorSistema>{
            let caller = self.env().caller();
            return self._generar_orden_compra(lista_publicaciones_con_cantidades, dinero_disponible, caller);
        }
        
        // Recibe un vector con las publicaciones y la cantidad de cada una para armar la orden.
        fn _generar_orden_compra(&mut self, lista_publicaciones_con_cantidades:Vec<(u128, u32)> , dinero_disponible:u32, caller:AccountId) -> Result<OrdenCompra, ErrorSistema>{
            // Checkeo si el usuario que esta tratando de realizar la compra tiene el rol debido
            
            self.es_vendedor()?;

            // Verifico que por lo menos exista una compra

            // Busco el id del vendedor
            let vendedor_actual:AccountId;
            if let Some(publi) = self.publicaciones.iter().find(|x|x.id_publicacion == lista_publicaciones_con_cantidades[0].0) {
                vendedor_actual = publi.id_publicador;
            }
            else {
                return Err(ErrorSistema::PublicacionNoValida)
            }

            //Si el usuario que creo la publicacion trata de realizar una compra hay error

            if vendedor_actual == caller {
                return Err(ErrorSistema::NoPuedeComprarPublicacionPropia);
            }

            
            self.validar_orden(lista_publicaciones_con_cantidades.clone(), vendedor_actual.clone())?;

            let monto_total = self.validar_precio(lista_publicaciones_con_cantidades.clone(), dinero_disponible)?;


            // Una vez pasadas todas las validaciones, actualizo el stock

            let lista_compra = self.actualizar_stock_de_orden(lista_publicaciones_con_cantidades);


            let id_orden = self.generar_id_orden()?;
            

            // Creo la orden

            let orden = OrdenCompra {
                id_comprador: caller,
                lista_productos: lista_compra,
                id_orden_compra: id_orden,
                estado: EstadoOrdenCompra::Pendiente,
                id_vendedor: vendedor_actual,
                solicitud_cancelacion: None,
                monto: monto_total,
            };
            
            // Agrego la orden al vector de ordenes
            self.ordenes.push(orden.clone());
        
            // Agrego al vector de ambos usuarios
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
            // Itero sobre la lista de publicaciones con cantidades y voy checkeando si la compra es valida(id de publicaciones valida y cant valida)

            let mut vistos = BTreeSet::new();
            
            for (id_publicacion_actual, cant_productos) in lista_publicaciones_con_cantidades {

                //Check de que no compre dos veces de la misma publicacion
                if !vistos.insert(id_publicacion_actual) {
                    return Err(ErrorSistema::PublicacionRepetida)
                }

                // Check que se trate de comprar aunque sea un item de la publicacion
                if cant_productos==0 {
                    return Err(ErrorSistema::NoPuedeComprarCero)
                }

                if let Some(publicacion_actual) = self.publicaciones.iter().find(|x| x.id_publicacion == id_publicacion_actual){

                    // Veo que todas las publicaciones sean del mismo vendedor
                    if publicacion_actual.id_publicador != vendedor_actual {
                        return Err(ErrorSistema::VendedorDistinto)
                    }

                    // Veo que la publicacion tengo el stock necesario para la compra
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
                    match monto_total.checked_add(monto_actual) {
                        Some(val) => monto_total = val,
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

        #[ink(message)]
        pub fn marcar_orden_como_enviada(&mut self, id_actual:u128)->Result<(), ErrorSistema> {
            let caller = self.env().caller();
            self._marcar_orden_como_enviada(id_actual, caller)
        }

        fn _marcar_orden_como_enviada(&mut self, id_actual:u128, caller:AccountId)->Result<(), ErrorSistema>{
            if let Some(pos) = self.ordenes.iter().position(|x| x.id_orden_compra==id_actual){

                if let Some(orden_acutal) = self.ordenes.get_mut(pos){
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
            else {
                return Err(ErrorSistema::IdDeOrdenNoValida);
            }
        }

        #[ink(message)]
        pub fn marcar_orden_como_recibida(&mut self, id_actual:u128)->Result<(), ErrorSistema> {
            let caller = self.env().caller();
            self._marcar_orden_como_recibida(id_actual, caller)
        }

        fn _marcar_orden_como_recibida(&mut self, id_actual:u128, caller:AccountId)->Result<(), ErrorSistema>{
            if let Some(pos) = self.ordenes.iter().position(|x| x.id_orden_compra==id_actual){

                if let Some(orden_acutal) = self.ordenes.get_mut(pos){
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
            else {
                return Err(ErrorSistema::IdDeOrdenNoValida);
            }
        }

        #[ink(message)]
        pub fn cancelar_orden(&mut self, id_actual:u128)->Result<(), ErrorSistema> {
            let caller = self.env().caller();
            self._cancelar_orden(id_actual, caller)
        }

        fn _cancelar_orden(&mut self, id_actual:u128, caller:AccountId) -> Result<(), ErrorSistema> {
            if let Some(pos) = self.ordenes.iter().position(|x| x.id_orden_compra==id_actual) {

                if let Some(orden_acutal) = self.ordenes.get_mut(pos) {

                    if let Some(id_anterior) = orden_acutal.solicitud_cancelacion {
                        if id_anterior == caller {
                            return Err(ErrorSistema::CancelacionYaSolicitada);
                        }
                        else {
                            if id_anterior == orden_acutal.id_comprador || id_anterior == orden_acutal.id_vendedor{

                                self.ordenes.get_mut(pos).unwrap().estado = EstadoOrdenCompra::Cancelado;
                                return Ok(())
                            }
                        }
                    }
                    self.ordenes.get_mut(pos).unwrap().solicitud_cancelacion = Some(caller);
                    return Ok(())
                    
                }
                else {
                    return Err(ErrorSistema::IdDeOrdenNoValida);
                }
            }
            else {
                return Err(ErrorSistema::IdDeOrdenNoValida);
            }
        }

        #[ink(message)]
        pub fn get_publicaciones(&self)->Vec<Publicacion>{
            self.publicaciones.clone()
        }

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
            if self.rol == rol {
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
        #[ink::test]
        fn registrar_usuario_okay() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();

            assert!(sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador).is_ok());
        }

        /// We test that we cannot register a user that already exists.
        #[ink::test]
        fn registrar_usuario_not_okay() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            assert!(sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador).is_err());
        }

        #[ink::test]
        fn test_existe_usuario() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            assert!(sistema._existe_usuario(alice).is_ok());

            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);
            assert!(sistema._existe_usuario(bob).is_err());
        }

        #[ink::test]
        fn test_es_vendedor() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Vendedor);

            //Pruebo con un usuario (alice) que esté en el sistema y sea vendedor.
            assert!(matches!(sistema.es_vendedor(), Ok(true)));

            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Comprador);

            //Pruebo con un usuario (charlie) que esté en el sistema pero no sea vendedor.
            assert!(matches!(sistema.es_vendedor(), Ok(false)));


            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            //Pruebo con un usuario (bob) que no esté en el sistema.
            assert!(sistema.es_vendedor().is_err());
        }

        #[ink::test]
        fn test_es_comprador() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            //Pruebo con un usuario (alice) que esté en el sistema y sea comprador.
            assert!(matches!(sistema.es_comprador(), Ok(true)));

            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            //Pruebo con un usuario (charlie) que esté en el sistema pero no sea vendedor.
            assert!(matches!(sistema.es_comprador(), Ok(false)));


            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            //Pruebo con un usuario (bob) que no esté en el sistema.
            assert!(sistema.es_comprador().is_err());
        }

        #[ink::test]
        fn test_agregar_rol() {
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
            //-----------------------------------------------------

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

            //-----------------------------------------------------

            //Inicializa charlie como vendedor.
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);

            let mut sistema = Sistema::new();
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            //Ya tiene el rol de vendedor. Por lo qe no se puede agregar el rol de vendedor otra vez..
            let error = sistema.agregar_rol(Rol::Vendedor).unwrap_err();
            assert_eq!(error, ErrorSistema::RolYaEnUso);

            //-----------------------------------------------------
            let eve = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().eve;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(eve);

            //Pruebo con un usuario (dave) que no esté en el sistema.
            let error = sistema.agregar_rol(Rol::Vendedor).unwrap_err();
            assert_eq!(error, ErrorSistema::UsuarioNoExiste);
        }

        #[ink::test]
        fn test_publicacion_tiene_stock(){
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            if let Ok(id) = sistema.nuevo_producto("banana".to_string(), "una banana".to_string(), Categoria::Limpieza){
                assert_eq!(id, 0);
            }

            sistema.crear_publicacion(0, 10, 19);
            assert_eq!(sistema.get_publicaciones().len(), 1);

            assert_eq!(sistema.publicaciones[0].tiene_stock_suficiente(20), false);

            let mut lista_compra = Vec::new();
            lista_compra.push((0,2));


            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            if let Err(e) = sistema.generar_orden_compra(lista_compra.clone(), 1) {
                assert_eq!(e, ErrorSistema::DineroInsuficiente);
            }
            
            assert!(sistema.generar_orden_compra(lista_compra, 200).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);

            assert!(sistema.marcar_orden_como_enviada(0).is_ok());
            if let Some(orden) = sistema.ordenes.get(0){
                assert_eq!(orden.estado, EstadoOrdenCompra::Enviado);
            }

            assert!(sistema.cancelar_orden(0).is_ok());

            if let Err(e) = sistema.cancelar_orden(0) {
                assert_eq!(e, ErrorSistema::CancelacionYaSolicitada);
            }

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            assert!(sistema.cancelar_orden(0).is_ok());

            if let Some(orden) = sistema.ordenes.get(0) {
                assert_eq!(orden.estado, EstadoOrdenCompra::Cancelado);
            }
            
        }

        #[ink::test]
        fn test_nuevo_producto_error() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Comprador);

            let error = sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros).unwrap_err();
            assert_eq!(error, ErrorSistema::UsuarioNoEsVendedor);
        }

        #[ink::test]
        fn test_crear_publicacion_errores() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;

            //let error_user_no_existe = sistema.crear_publicacion(0, 1000, 4).unwrap_err();
            //assert_eq!(error_user_no_existe, ErrorSistema::UsuarioNoExiste); //No existe el usuario que llama a la función.

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Comprador);

            let error_user_no_vendedor = sistema.crear_publicacion(0, 1000, 4).unwrap_err();
            assert_eq!(error_user_no_vendedor, ErrorSistema::UsuarioNoEsVendedor); //Ok.

            sistema.agregar_rol(Rol::Vendedor).unwrap(); //Agrego el rol de vendedor a Charlie.
            let error_producto_invalido = sistema.crear_publicacion(0, 1000, 4).unwrap_err();
            assert_eq!(error_producto_invalido, ErrorSistema::ProductoInvalido); //No existe el producto con id 0.

        }

        #[ink::test]
        fn test_generar_orden_compra_error() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            //Quiero forzar el error de publicacionNoValida
            let error_publicacion_invalida = sistema.generar_orden_compra(vec![(0, 1)],1).unwrap_err();
            assert_eq!(error_publicacion_invalida, ErrorSistema::PublicacionNoValida); //Ok

            //Quiero forzar el error de NoPuedeComprarPublicacionPropia
            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4);

            let error_no_puede_comprar_publicacion_propia = sistema.generar_orden_compra(vec![(0, 1)],4000).unwrap_err();
            assert_eq!(error_no_puede_comprar_publicacion_propia, ErrorSistema::NoPuedeComprarPublicacionPropia); //Ok.
        }

        #[ink::test]
        fn test_agregar_orden_usuario() {
            let mut sistema = Sistema::new();

            let error_usuario_no_existe = sistema.agregar_orden_usuario(AccountId::from([0x1; 32]), 1).unwrap_err();
            assert_eq!(error_usuario_no_existe, ErrorSistema::UsuarioNoExiste); //No existe el usuario que llama a la función.
        }

        #[ink::test]
        fn test_validar_orden_errores() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de PublicacionRepetida.
            let error_publicacion_repetida = sistema.validar_orden(vec![(0, 1), (0, 2)], charlie).unwrap_err(); 
            assert_eq!(error_publicacion_repetida, ErrorSistema::PublicacionRepetida); 

            //Quiero forzar el error de NoPuedeComprarCero.
            let error_no_puede_comprar_cero = sistema.validar_orden(vec![(0, 0)], charlie).unwrap_err();
            assert_eq!(error_no_puede_comprar_cero, ErrorSistema::NoPuedeComprarCero); //Ok.


            //Quiero forzar el error de VendedorDistinto.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let error_vendedor_distinto = sistema.validar_orden(vec![(0, 1)], alice).unwrap_err();
            assert_eq!(error_vendedor_distinto, ErrorSistema::VendedorDistinto); //Ok.

            //Quiero forzar el error de StockInsuficiente.
            //Vuelvo a que el caller sea charlie.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            let error_stock_insuficiente = sistema.validar_orden(vec![(0, 5)], charlie).unwrap_err();
            assert_eq!(error_stock_insuficiente, ErrorSistema::StockInsuficiente); //Ok.

            //Quiero forzar el error de PublicacionNoValida.
            let error_publicacion_invalida = sistema.validar_orden(vec![(1, 1)], charlie).unwrap_err();
            assert_eq! (error_publicacion_invalida, ErrorSistema::PublicacionNoValida); //Ok.
        }

        #[ink::test]
        fn test_marcar_orden_como_enviada_errores() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de IdDeOrdenNoValida.
            let error_id_invalido = sistema.marcar_orden_como_enviada(0).unwrap_err();
            assert_eq!(error_id_invalido, ErrorSistema::IdDeOrdenNoValida); //No existe la orden con id 0.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);
            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());

            //Quiero forzar el error de OperacionNoValida.
            let error_operacion_no_valida = sistema.marcar_orden_como_enviada(0).unwrap_err();
            assert_eq!(error_operacion_no_valida, ErrorSistema::OperacionNoValida); //El caller no es el vendedor de la orden.

            //Quiero forzar el error de OperacionNoValida porque la orden ya fue enviada.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            assert!(sistema.marcar_orden_como_enviada(0).is_ok()); //Primero lo marco como enviada.
            let error_operacion_no_valida = sistema.marcar_orden_como_enviada(0).unwrap_err();
            assert_eq!(error_operacion_no_valida, ErrorSistema::OperacionNoValida); //La orden ya fue enviada.

        }

        #[ink::test]
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
            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());


            //Quiero marcar la orden como recibida.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.marcar_orden_como_enviada(0); //primero lo marco como enviada

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            assert_eq!(sistema.marcar_orden_como_recibida(0), Ok(()));
        }

        #[ink::test]
        fn test_marcar_orden_como_recibida_errores() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

            //Quiero forzar el error de IdDeOrdenNoValida.
            let error_id_invalido = sistema.marcar_orden_como_enviada(0).unwrap_err();
            assert_eq!(error_id_invalido, ErrorSistema::IdDeOrdenNoValida); //No existe la orden con id 0.

            //Creo una orden de compra para que exista una orden con id 0.
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);
            let lista_compra = vec![(0, 1)];
            assert!(sistema.generar_orden_compra(lista_compra,4000).is_ok());

            //Quiero forzar el error de OperacionNoValida.
            let error_operacion_no_valida = sistema.marcar_orden_como_recibida(0).unwrap_err();
            assert_eq!(error_operacion_no_valida, ErrorSistema::OperacionNoValida); //El caller no es el vendedor de la orden.

            //Quiero forzar el error de OperacionNoValida porque la orden ya fue recibida.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.marcar_orden_como_enviada(0); //Primero lo marco como enviada.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            assert!(sistema.marcar_orden_como_recibida(0).is_ok()); //Primero lo marco como recibida.
            let error_operacion_no_valida = sistema.marcar_orden_como_recibida(0).unwrap_err();
            assert_eq!(error_operacion_no_valida, ErrorSistema::OperacionNoValida); //La orden ya fue recibida.
            

        }
        
        #[ink::test]
        fn test_cancelar_orden_errores() {
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
        fn test_ver_mis_ordenes() {
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Ambos);

            sistema.nuevo_producto("Termo".to_string(), "Termo de metal".to_string(), Categoria::Otros);
            sistema.crear_publicacion(0, 1000, 4); //La publicación la crea Charlie.

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
        fn test_calculo_precio(){
            let mut sistema = Sistema::new();
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);
            sistema.nuevo_producto("Cif".to_string(), "Cif".to_string(), Categoria::Limpieza);
            sistema.nuevo_producto("Remera".to_string(), "Remera".to_string(), Categoria::Ropa);
            sistema.crear_publicacion(0, 10, 19);
            sistema.crear_publicacion(1, 20, 5);

            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Ambos);

            let mut lista_compra = Vec::new();
            lista_compra.push((0,2));
            lista_compra.push((1,3));


            if let Err(e) = sistema.generar_orden_compra(lista_compra.clone(), 70){
                assert_eq!(e, ErrorSistema::DineroInsuficiente);
            }

            if let Ok(ord) = sistema.generar_orden_compra(lista_compra, 200){
                assert_eq!(ord.monto, 80);
            }


        }
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
