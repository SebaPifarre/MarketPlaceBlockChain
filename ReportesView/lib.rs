#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ReportesView {
    use ink::codegen::TraitCallBuilder;
    use MarketPlace::SistemaRef;

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

        
    }
}
