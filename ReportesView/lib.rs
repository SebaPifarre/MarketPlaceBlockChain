#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ReportesView {
    use ink::codegen::TraitCallBuilder;
    use ink::prelude::vec::Vec;
    
    use MarketPlace::{
        SistemaRef,
        Usuario,
        ErrorSistema,
        Categoria
    };

    #[ink(storage)]
    pub struct ReportesView {
        marketplace: SistemaRef
    }

    impl ReportesView {

        // Instancia el marketplace y lo devuelve contenido en una estructura
        // para referenciarlo
        #[ink(constructor)]
        pub fn new(marketplace_code_hash: Hash) -> Self {
            let marketplace = SistemaRef::new()
                .code_hash(marketplace_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF]) // Salt arbitrario
                .instantiate();

            Self { marketplace }
        }

        /// Devuelve hasta 5 usuarios (Vendedor/Ambos) ordenados por puntaje como vendedor.
        /// Solo accesible si se ha establecido ReportesView.
        #[ink(message)]
        pub fn get_top5_vendedores(&self) -> Result<Vec<Usuario>,ErrorSistema> {
            self.marketplace.consultar_top_5_vendedores()
        }

        /// Devuelve hasta 5 usuarios (Vendedor/Ambos) ordenados por puntaje como vendedor.
        /// Solo accesible si se ha establecido ReportesView.
        #[ink(message)]
        pub fn get_top5_compradores(&self) -> Result<Vec<Usuario>,ErrorSistema> {
            self.marketplace.consultar_top_5_compradores()
        }
        
        /// Devuelve un vector con las estadisticas por categoría, con la cantidad total de ventas y la suma de puntuaciones.
        #[ink(message)]
        pub fn get_estadisticas_por_categoria(&self) -> Result<Vec<(Categoria, u32, u8)>, ErrorSistema> { 
            self.marketplace.estadisticas_por_categoria()
        }

        /// Devuelve una lista de hasta 10 productos de una categoría específica ordenados por ventas en forma descendente.
        /// Solo accesible si se ha establecido ReportesView.
        ///
        /// # Parámetros
        /// - `categoria`: Categoría de los productos a buscar.
        ///
        #[ink(message)]
        pub fn get_productos_mas_vendidos(&self, categoria: Categoria) -> Result<Vec<(u128, u8)>, ErrorSistema> {
            self.marketplace.ver_productos_mas_vendidos(categoria)
        }

        /// Devuelve un vector con la cantidad de órdenes de todos los usuarios registrados.
        #[ink(message)]
        pub fn get_ordenes_por_usuario(&self) -> Result<Vec<(AccountId, u128)>, ErrorSistema> {
            self.marketplace.cantidad_ordenes_por_usuario()
        }

    }
}
