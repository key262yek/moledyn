extern crate proc_macro;
extern crate proc_quote;

extern crate syn;

#[derive(Debug)]
struct ParsedArguments{
    prefix      : proc_macro2::Literal,
    setups      : Vec<proc_macro2::Ident>,
}

impl syn::parse::Parse for ParsedArguments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut parsed_args = ParsedArguments {
            prefix      : proc_macro2::Literal::string("RTS_GENERAL_"),
            setups      : vec![],
        };

        if !input.is_empty() {
            parsed_args.prefix = match input.parse() {
                Ok(prefix) => prefix,
                Err(err) => return Err(syn::Error::new(err.span(), format!(
                    "The first token must be an Literal"))),
            };

            // 그 다음부터는 comma (,) 와 인자 타입을 의미하는 identifier 가 반복해서 와야한다.
            let mut i = 1;
            while !input.is_empty() {
                // comma 인지 체크
                match input.parse::<syn::token::Comma>() {
                    Ok(_) => {},
                    Err(err) => return Err(syn::Error::new(err.span(), format!(
                        "The token {} must be a comma.", i + 1))),
                };
                i += 1;

                if input.is_empty() {
                    break;
                }

                // Identifier 인지 체크
                let type_name = match input.parse::<syn::Ident>() {
                    Ok(type_name) => type_name,
                    Err(err) => return Err(syn::Error::new(err.span(), format!(
                        "The token {} must be an identifier.", i + 1))),
                };
                parsed_args.setups.push(type_name);
                i += 1;
            }
        }

        if parsed_args.setups.len() != 6{
            return Err(syn::Error::new(proc_macro2::Span::call_site(), format!("Number of identifier tokens should be six.")));
        }

        Ok(parsed_args)
    }
}

#[allow(dead_code)]
fn ident_to_dataset(ident : &proc_macro2::Ident) -> proc_macro2::TokenStream{
    match ident.to_string().as_str() {
        // System Types
        "ContCircSystem" => {
            let tokens = proc_quote::quote!{
                ContCircSystem, sys_arg, ContCircSystemArguments, [sys_size, f64, dim, usize]
            };
            tokens.into()
        },
        "ContCubicSystem" => {
            let tokens = proc_quote::quote!{
                ContCubicSystem, sys_arg, ContCubicSystemArguments, [sys_size, f64, dim, usize]
            };
            tokens.into()
        },
        "ContCylindricalSystem" => {
            let tokens = proc_quote::quote!{
                ContCylindricalSystem, sys_arg, ContCylindricalSystemArguments, [radius, f64, length, f64, dim, usize]
            };
            tokens.into()
        },

        // Target Types
        "ContBoundaryTarget" => {
            let tokens = proc_quote::quote!{
                ContBoundaryTarget, target_arg, ContBoundaryTargetArguments, [target_size, f64]
            };
            tokens.into()
        },
        "ContBulkTarget" => {
            let tokens = proc_quote::quote!{
                ContBulkTarget, target_arg, ContBulkTargetArguments, [target_size, f64]
            };
            tokens.into()
        },

        // Searcher Types
        "ContPassiveIndepSearcher" => {
            let tokens = proc_quote::quote!{
                ContPassiveIndepSearcher, searcher_arg, ContPassiveIndepSearcherArguments, [num_searcher, usize]
            };
            tokens.into()
        },
        "ContPassiveMergeSearcher" => {
            let tokens = proc_quote::quote!{
                ContPassiveMergeSearcher, searcher_arg, ContPassiveMergeSearcherArguments, [radius, f64, alpha, f64, num_searcher, usize]
            };
            tokens.into()
        },
        "ContPassiveExpSearcher" => {
            let tokens = proc_quote::quote!{
                ContPassiveExpSearcher, searcher_arg, ContPassiveExpSearcherArguments, [gamma, f64, strength, f64, num_searcher, usize]
            };
            tokens.into()
        },
        "ContPassiveLJSearcher" => {
            let tokens = proc_quote::quote!{
                ContPassiveLJSearcher, searcher_arg, ContPassiveLJSearcherArguments, [ptl_size, f64, strength, f64, num_searcher, usize]
            };
            tokens.into()
        },

        // Timestep Types
        "ConstStep" => {
            let tokens = proc_quote::quote!{
                ConstStep, time_arg, ConstStepArguments, [dt, f64, tmax, f64]
            };
            tokens.into()
        },
        "ExponentialStep" => {
            let tokens = proc_quote::quote!{
                ExponentialStep, time_arg, ExponentialStepArguments, [dt_min, f64, dt_max, f64, length, usize, tmax, f64]
            };
            tokens.into()
        },

        // Simulation types
        "VariableSimulation" => {
            let tokens = proc_quote::quote!{
                VariableSimulation, sim_arg, VariableSimulationArguments, [idx_set, usize]
            };
            tokens.into()
        },
        "ProcessSimulation" => {
            let tokens = proc_quote::quote!{
                ProcessSimulation, sim_arg, ProcessSimulationArguments, [period, f64, idx_set, usize]
            };
            tokens.into()
        },
        _ => {
            let tokens = proc_quote::quote!{
                #ident
            };
            tokens.into()
        }
    }
}



impl ParsedArguments{
    #[allow(dead_code)]
    fn proc_construct_dataset(&self) -> proc_macro2::TokenStream{
        let tt_sys = ident_to_dataset(&self.setups[1]);
        let tt_target = ident_to_dataset(&self.setups[2]);
        let tt_searcher = ident_to_dataset(&self.setups[3]);
        let tt_time = ident_to_dataset(&self.setups[4]);
        let tt_sim = ident_to_dataset(&self.setups[5]);

        let tokens = proc_quote::quote!{
            construct_dataset!(SimulationData,
            #tt_sys; #tt_target; #tt_searcher; #tt_time; {#tt_sim});
        };
        tokens.into()
    }

    #[allow(dead_code)]
    fn proc_setup_simulation(&self) -> proc_macro2::TokenStream{
        let prefix = self.prefix.clone();
        let id_analysis = self.setups[0].clone();
        let id_sys = self.setups[1].clone();
        let id_target = self.setups[2].clone();
        let id_searcher = self.setups[3].clone();
        let id_time = self.setups[4].clone();
        let id_sim = self.setups[5].clone();

        let tokens = proc_quote::quote!{
            setup_simulation!(args, 15, 1, #id_analysis, #prefix,
                dataset, SimulationData, sys_arg, #id_sys,
                target_arg, #id_target, searcher_arg, #id_searcher,
                time_arg, #id_time, sim_arg, #id_sim);
        };
        tokens.into()
    }

    #[allow(dead_code)]
    fn proc_variable_declare(&self) -> proc_macro2::TokenStream{
        fn ident_to_variables(ident : proc_macro2::Ident) -> Vec<proc_macro2::Ident>{
            fn string_to_ident(name : &str) -> proc_macro2::Ident{
                syn::Ident::new(name, proc_macro2::Span::call_site())
            }

            let mut vec : Vec<proc_macro2::Ident> = Vec::new();

            match ident.to_string().as_str() {
                "ContCircSystem" |
                "ContCubicSystem" => {
                    vec.push(string_to_ident("sys_size"));
                    vec.push(string_to_ident("dim"));
                },
                "ContCylindricalSystem" => {
                    vec.push(string_to_ident("sys_radius"));
                    vec.push(string_to_ident("sys_length"));
                    vec.push(string_to_ident("dim"));
                },

                "ContBoundaryTarget"  |
                "ContBulkTarget" => {
                    vec.push(string_to_ident("target_pos"));
                    vec.push(string_to_ident("target_size"));
                },

                "ContPassiveIndepSearcher" => {
                    vec.push(string_to_ident("mtype"));
                    vec.push(string_to_ident("itype"));
                    vec.push(string_to_ident("num_searcher"));
                },
                "ContPassiveMergeSearcher" => {
                    vec.push(string_to_ident("mtype"));
                    vec.push(string_to_ident("itype"));
                    vec.push(string_to_ident("ptl_radius"));
                    vec.push(string_to_ident("alpha"));
                    vec.push(string_to_ident("num_searcher"));
                },
                "ContPassiveExpSearcher" => {
                    vec.push(string_to_ident("mtype"));
                    vec.push(string_to_ident("itype"));
                    vec.push(string_to_ident("gamma"));
                    vec.push(string_to_ident("exp_dim"));
                    vec.push(string_to_ident("strength"));
                    vec.push(string_to_ident("num_searcher"));
                },
                "ContPassiveLJSearcher" => {
                    vec.push(string_to_ident("mtype"));
                    vec.push(string_to_ident("itype"));
                    vec.push(string_to_ident("ptl_size"));
                    vec.push(string_to_ident("strength"));
                    vec.push(string_to_ident("num_searcher"));
                },

                // Timestep Types
                "ConstStep" => {
                    vec.push(string_to_ident("dt"));
                    vec.push(string_to_ident("tmax"));
                },
                "ExponentialStep" => {
                    vec.push(string_to_ident("dt_min"));
                    vec.push(string_to_ident("dt_max"));
                    vec.push(string_to_ident("length"));
                    vec.push(string_to_ident("tmax"));
                },

                // Simulation types
                "VariableSimulation" => {
                    vec.push(string_to_ident("num_ensemble"));
                    vec.push(string_to_ident("idx_set"));
                    vec.push(string_to_ident("seed"));
                    vec.push(string_to_ident("output_dir"));
                },
                "ProcessSimulation" => {
                    vec.push(string_to_ident("num_ensemble"));
                    vec.push(string_to_ident("period"));
                    vec.push(string_to_ident("idx_set"));
                    vec.push(string_to_ident("seed"));
                    vec.push(string_to_ident("output_dir"));
                },

                _ => {}
            }
            vec
        }

        let id_sys = self.setups[1].clone();
        let var_sys = ident_to_variables(id_sys);

        let id_target = self.setups[2].clone();
        let var_target = ident_to_variables(id_target);

        let id_searcher = self.setups[3].clone();
        let var_searcher = ident_to_variables(id_searcher);

        let id_time = self.setups[4].clone();
        let var_time = ident_to_variables(id_time);

        let id_sim = self.setups[5].clone();
        let var_sim = ident_to_variables(id_sim);

        let tokens = proc_quote::quote!{
            #(let #var_sys = sys_arg.#var_sys.clone();)*
            #(let #var_target = target_arg.#var_target.clone();)*
            #(let #var_searcher = searcher_arg.#var_searcher.clone();)*
            #(let #var_time = time_arg.#var_time.clone();)*
            #(let #var_sim = sim_arg.#var_sim.clone();)*
        };
        tokens.into()
    }

    #[allow(dead_code)]
    fn proc_export_simulation_info(&self) -> proc_macro2::TokenStream{
        let prefix = self.prefix.clone();
        let id_sys = self.setups[1].clone();
        let id_target = self.setups[2].clone();
        let id_searcher = self.setups[3].clone();
        let id_time = self.setups[4].clone();
        let id_sim = self.setups[5].clone();

        let tokens = proc_quote::quote!{
            export_simulation_info!(dataset, output_dir, writer, WIDTH,
                #prefix,
                #id_sys, sys, sys_arg,
                #id_target, target, target_arg,
                #id_searcher, vec_searchers, searcher_arg,
                #id_time, timeiter, time_arg,
                #id_sim, simulation, sim_arg);
        };
        tokens.into()
    }
}


#[proc_macro]
pub fn simulation(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let ident = syn::parse_macro_input!(input as ParsedArguments);

    let token_cd = ident.proc_construct_dataset();
    let token_setup = ident.proc_setup_simulation();
    let token_var = ident.proc_variable_declare();
    let token_export = ident.proc_export_simulation_info();

    let tokens = proc_quote::quote!{
        #token_cd
        #token_setup
        #token_var
        #token_export
    };
    tokens.into()
}





#[proc_macro]
pub fn check_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let ident = syn::parse_macro_input!(input as ParsedArguments);

    let prefix = ident.prefix;
    let setups = &ident.setups;

    let tokens = proc_quote::quote!{
        println!("{:?}", #prefix);
        #(println!(stringify!(#setups));)*
    };
    tokens.into()
}

#[proc_macro]
pub fn test_input(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ident = syn::parse_macro_input!(input as proc_macro2::Ident);

    let tokens = proc_quote::quote!{
        println!(stringify!(#ident));
    };
    tokens.into()
}

// #[proc_macro]
// pub fn test_token_matching(input : proc_macro::TokenStream) -> proc_macro::TokenStream{
//     let ident = syn::parse_macro_input!(input as proc_macro2::Ident);
//     let t = ident_to_dataset(&ident);

//     let tokens = proc_quote::quote!{
//         let a = (#t);
//     };
//     tokens.into()
// }

#[proc_macro]
pub fn test_proc_macros(input : proc_macro::TokenStream) -> proc_macro::TokenStream{
    let ident = syn::parse_macro_input!(input as ParsedArguments);
    let ds_token = ident.proc_construct_dataset();

    let tokens = proc_quote::quote!{
        #ds_token
    };
    tokens.into()
}


fn match_ident(ident : proc_macro2::Ident) -> proc_macro::TokenStream{
    match ident.to_string().as_str() {
        "Should_matched" => {
            let tt = syn::Ident::new("Matched", proc_macro2::Span::call_site());
            let tokens = proc_quote::quote!{
                println!(stringify!(#tt));
            };
            tokens.into()
        }
        _ => {
            let tokens = proc_quote::quote!{
                println!(stringify!(#ident));
            };
            tokens.into()
        }
    }
}


#[proc_macro]
pub fn test_match(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let dataset = syn::parse_macro_input!(input as proc_macro2::Ident);
    match_ident(dataset)
}
