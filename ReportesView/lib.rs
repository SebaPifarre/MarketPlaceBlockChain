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

        #[ink(message)]
        pub fn obtener_top_compradores(&self) -> Result<Vec<Usuario>,ErrorSistema> {
            self.marketplace.consultar_top_5_compradores()
        }

        #[ink(message)]
        pub fn obtener_top_vendedores(&self) -> Result<Vec<Usuario>,ErrorSistema> {
            self.marketplace.consultar_top_5_vendedores()
        }

        #[ink(message)]
        pub fn obtener_estadisticas_por_categoria(&self) -> Result<Vec<(Categoria, u32, u8)>, ErrorSistema> { 
            self.marketplace.estadisticas_por_categoria()
        }

        #[ink(message)]
        pub fn ver_productos_mas_vendidos(&self, categoria: Categoria) -> Result<Vec<(u128, u8)>, ErrorSistema> {
            self.marketplace.ver_productos_mas_vendidos(categoria)
        }

        #[ink(message)]
        pub fn cantidad_ordenes_por_usuario(&self) -> Result<Vec<(AccountId, u128)>, ErrorSistema> {
            self.marketplace.cantidad_ordenes_por_usuario()
        }

    }
}
