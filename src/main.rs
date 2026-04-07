use std::collections::HashMap;

use egui::CollapsingHeader;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Browser Controller",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            let mut b = Box::<BrowserControllerApp>::default();
            b.compute_content();
            Ok(b)
        }),
    )
}

struct BrowserControllerApp {
    // list of filter modes and contained list of tags
    filters:Vec<BrowserControllerFilter>,
    filter_idx:usize,
    // list of items with metadata
    items:HashMap<String,BrowserControllerItem>,
    // join list of group to items relationships
    joins:Vec<BrowserControllerItemGroupJoin>,
    // view cache?
    content:Vec<BrowserControllerContentItem>,
}
#[derive(Clone)]
struct BrowserControllerFilter{
    name:String,
    groups:Vec<String>
}
#[derive(Clone)]
struct BrowserControllerItem{
    name:String,
    website_link:Option<String>,
    notes:String,
}
#[derive(Clone)]
struct BrowserControllerItemGroupJoin{
    item_name:String,
    group_name:String,
}
#[derive(Clone)]
struct BrowserControllerContentItem{
    group_name:String,
    items:Vec<BrowserControllerItem>,
}

impl Default for BrowserControllerApp {
    fn default() -> Self {
        // empty
        // Self {
        //     filters: vec![],
        //     filter_idx:0,
        //     items: vec![],
        //     joins: vec![],
        // }
        // mock
        Self { 
            filters: vec![
                BrowserControllerFilter{
                    name:"Learning".into(),
                    groups:vec![
                        "Japanese".into(),
                        "Embedded".into(),
                        "Rust".into(),
                        "Godot".into(),
                        "Cooking".into(),
                        ]
                },
                BrowserControllerFilter{
                    name:"Job".into(),
                    groups:vec![
                        "MCreator".into(),
                        "Roblox".into(),
                        "Python".into(),
                        "Godot".into(),
                        "Useful Links".into(),
                    ]
                },
                BrowserControllerFilter{
                    name:"Hobbies".into(),
                    groups:vec![
                        "Japanese".into(),
                        "Anime".into(),
                        "Video Games".into(),
                        "Trains".into(),
                        "Music".into(),
                    ]
                }
            ], 
            filter_idx: 2, 
            items: HashMap::from([
                ("WaniKani".into(),BrowserControllerItem{
                    name:"WaniKani".into(),
                    website_link:Some("wanikani.com".into()),
                    notes:"".into(),
                }),
                ("Migaku".into(),BrowserControllerItem{
                    name:"Migaku".into(),
                    website_link:Some("study.migaku.com".into()),
                    notes:"".into()
                }),
                ("Crunchyroll".into(),BrowserControllerItem{
                    name:"Crunchyroll".into(),
                    website_link:Some("crunchyroll.com".into()),
                    notes:"".into()
                }),
                ("Jisho".into(),BrowserControllerItem{
                    name:"Jisho".into(),
                    website_link:Some("jisho.org".into()),
                    notes:"".into()
                }),
                ("STM".into(),BrowserControllerItem{
                    name:"STM".into(),
                    website_link:Some("st.com".into()),
                    notes:"".into()
                }),
                ("Arduino".into(),BrowserControllerItem{
                    name:"Arduino".into(),
                    website_link:None,
                    notes:"".into()
                }),
                ("Embedded Course".into(),BrowserControllerItem{
                    name:"Embedded Course".into(),
                    website_link:Some("https://www.youtube.com/@StateMachineCOM".into()),
                    notes:"".into()
                }),
                ("Egui".into(),BrowserControllerItem{
                    name:"Egui".into(),
                    website_link:Some("github.com/emilk/egui".into()),
                    notes:"".into()
                }),
                ("Rust Book".into(),BrowserControllerItem{
                    name:"Rust Book".into(),
                    website_link:Some("doc.rust-lang.org/stable/book".into()),
                    notes:"".into()
                }),
                ("Godot Docs".into(),BrowserControllerItem{
                    name:"Godot Docs".into(),
                    website_link:Some("docs.godotengine.org/en/stable".into()),
                    notes:"".into()
                }),
                ("Cookies Recipe".into(),BrowserControllerItem{
                    name:"Cookies Recipe".into(),
                    website_link:None,
                    notes:"".into()
                }),
                ("MCreator Tutorial Playlist".into(),BrowserControllerItem{
                    name:"MCreator Tutorial Playlist".into(),
                    website_link:None,
                    notes:"".into()
                }),
                ("MCreator Download".into(),BrowserControllerItem{
                    name:"MCreator Download".into(),
                    website_link:Some("mcreator.net".into()),
                    notes:"".into(),
                }),
                ("Roblox Creator Dashboard".into(),BrowserControllerItem{
                    name:"Roblox Creator Dashboard".into(),
                    website_link:Some("create.roblox.com".into()),
                    notes:"".into()
                }),
                ("Zoom Link".into(),BrowserControllerItem{
                    name:"Zoom Link".into(),
                    website_link:None,
                    notes:"".into()
                }),
                ("Curriculum Folder".into(),BrowserControllerItem{
                    name:"Curriculum Folder".into(),
                    website_link:None,
                    notes:"".into()
                }),
                ("My Anime List".into(),BrowserControllerItem{
                    name:"My Anime List".into(),
                    website_link:Some("myanimelist.net".into()),
                    notes:"".into()
                }),
                ("Steam".into(),BrowserControllerItem{
                    name:"Steam".into(),
                    website_link:Some("store.steampowered.com".into()),
                    notes:"".into()
                }),
                ("North Shore Model Railroad Club".into(),BrowserControllerItem{
                    name:"North Shore Model Railroad Club".into(),
                    website_link:Some("nsmrc.org".into()),
                    notes:"".into()
                }),
                ("Spotify".into(),BrowserControllerItem{
                    name:"Spotify".into(),
                    website_link:Some("open.spotify.com".into()),
                    notes:"".into()
                })
            ]), 
            joins: vec![
                BrowserControllerItemGroupJoin{
                    item_name:"WaniKani".into(),
                    group_name:"Japanese".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Migaku".into(),
                    group_name:"Japanese".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Crunchyroll".into(),
                    group_name:"Japanese".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Crunchyroll".into(),
                    group_name:"Anime".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Jisho".into(),
                    group_name:"Japanese".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"STM".into(),
                    group_name:"Embedded".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Arduino".into(),
                    group_name:"Embedded".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Embedded Course".into(),
                    group_name:"Embedded".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Egui".into(),
                    group_name:"Rust".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Rust Book".into(),
                    group_name:"Rust".into()
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Godot Docs".into(),
                    group_name:"Godot".into()
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Cookies Recipe".into(),
                    group_name:"Cooking".into()
                },
                BrowserControllerItemGroupJoin{
                    item_name:"MCreator Tutorial Playlist".into(),
                    group_name:"MCreator".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"MCreator Download".into(),
                    group_name:"MCreator".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Roblox Creator Dashboard".into(),
                    group_name:"Roblox".into()
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Zoom Link".into(),
                    group_name:"Useful Links".into()
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Curriculum Folder".into(),
                    group_name:"Useful Links".into()
                },
                BrowserControllerItemGroupJoin{
                    item_name:"My Anime List".into(),
                    group_name:"Anime".into(),
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Steam".into(),
                    group_name:"Video Games".into()
                },
                BrowserControllerItemGroupJoin{
                    item_name:"North Shore Model Railroad Club".into(),
                    group_name:"Trains".into()
                },
                BrowserControllerItemGroupJoin{
                    item_name:"Spotify".into(),
                    group_name:"Music".into()
                }
            ],
            content: vec![]
        }
    }
}

impl eframe::App for BrowserControllerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx,|ui|{
            // Filter Selector
            ui.horizontal(|ui|{
                if ui.radio_value(&mut self.filter_idx, 0, "All").changed() {
                    self.compute_content();
                };
                for i in 0..self.filters.len(){
                    if ui.radio_value(&mut self.filter_idx, i + 1, self.filters[i].name.clone()).changed(){
                        self.compute_content();
                    };
                }   
            });
            // Content
            for group in &self.content{
                CollapsingHeader::new(group.group_name.clone())
                .default_open(false)
                .show(ui,|ui|{
                    for item in &group.items{
                        ui.horizontal(|ui|{
                            ui.label(item.name.clone());
                            if ui.button("Go").clicked(){
                                
                            }
                        });
                        ui.label(item.notes.clone());
                    }
                });
            }

        });
    }
}

impl BrowserControllerApp{
    fn compute_content(&mut self){
        self.content = match self.filter_idx{
            0=> vec![
                BrowserControllerContentItem{
                    group_name:"".into(),
                    items: self.items.values().map(|i|i.clone()).collect()
                }],
            _=> {
                let filter = &self.filters[self.filter_idx-1];
                let mut out = vec![];
                for group in &filter.groups{
                    out.push(BrowserControllerContentItem{
                        group_name:group.clone(),
                        items: self.join_on_group(group.clone())
                    });
                }
                out
            }
        }
    }

    fn join_on_group(&self,group_name:String)->Vec<BrowserControllerItem>{
        self.joins.iter().filter_map(|v|{
            if v.group_name==group_name{
                if let Some(item) = self.items.get(&v.item_name){
                    Some(item.clone())
                }else{
                    None
                }
            }else{
                None
            }
        }).collect()
    }
}