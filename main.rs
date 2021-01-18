pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}
pub struct CommandBuilder {
    executable: ::std::option::Option<String>,
    args: ::std::option::Option<String>,
    env: ::std::option::Option<String>,
    current_dir: ::std::option::Option<String>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for CommandBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            CommandBuilder {
                executable: ref __self_0_0,
                args: ref __self_0_1,
                env: ref __self_0_2,
                current_dir: ref __self_0_3,
            } => {
                let mut debug_trait_builder = f.debug_struct("CommandBuilder");
                let _ = debug_trait_builder.field("executable", &&(*__self_0_0));
                let _ = debug_trait_builder.field("args", &&(*__self_0_1));
                let _ = debug_trait_builder.field("env", &&(*__self_0_2));
                let _ = debug_trait_builder.field("current_dir", &&(*__self_0_3));
                debug_trait_builder.finish()
            }
        }
    }
}
impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: None,
            args: None,
            env: None,
            current_dir: None,
        }
    }
}
impl CommandBuilder {
    pub fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = ::std::option::Option::Some(executable);
        self
    }
    pub fn arg(&mut self, arg: String) -> &mut Self {
        if let Some(ref mut options) = self.args {
            options.push(arg);
        } else {
            self.args = ::std::option::Option::Some(<[_]>::into_vec(box [arg]));
        }
        self
    }
    pub fn env(&mut self, env: String) -> &mut Self {
        if let Some(ref mut options) = self.env {
            options.push(env);
        } else {
            self.env = ::std::option::Option::Some(<[_]>::into_vec(box [env]));
        }
        self
    }
    pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
        self.current_dir = ::std::option::Option::Some(current_dir);
        self
    }
    pub fn build(
        &mut self,
    ) -> ::std::result::Result<Command, ::std::boxed::Box<dyn ::std::error::Error>> {
        let executable = self
            .executable
            .clone()
            .ok_or("a mandatory field is missing :")?;
        let args = self.args.clone().ok_or("a mandatory field is missing :")?;
        let env = self.env.clone().ok_or("a mandatory field is missing :")?;
        let current_dir = self.current_dir.clone();
        ::std::result::Result::Ok(Command {
            executable,
            args,
            env,
            current_dir,
        })
    }
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();
}
