macro_rules! methods {
    ($($method:ident,)+) => {
        $(
            fn $method<T: for<'de> serde::Deserialize<'de>>(&self, url: String)
            -> Result<T>
            {
                let response = self.send(
                        &mut self.client.$method(&url)
                )?;

                deserialise(response)
            }
         )+
    };
}

macro_rules! paged_routes {

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            fn $name(&self) -> Result<Page<$ret, H>> {
                let url = self.route(concat!("/api/v1/", $url));
                let response = self.send(
                        &mut self.client.$method(&url)
                )?;

                Page::new(self, response)
            }

        }

        paged_routes!{$($rest)*}
    };

    () => {}
}

macro_rules! route {

    ((post multipart ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
                use reqwest::multipart::Form;

                let form_data = Form::new()
                    $(
                        .file(stringify!($param), $param.as_ref())?
                     )*;

                let response = self.send(
                        self.client
                            .post(&self.route(concat!("/api/v1/", $url)))
                            .multipart(form_data)
                )?;

                let status = response.status().clone();

                if status.is_client_error() {
                    return Err(Error::Client(status));
                } else if status.is_server_error() {
                    return Err(Error::Server(status));
                }

                deserialise(response)
            }
        }

        route!{$($rest)*}
    };

    (($method:ident ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),

            fn $name(&self, $($param: $typ,)*) -> Result<$ret> {

                let form_data = json!({
                    $(
                        stringify!($param): $param,
                    )*
                });

                let response = self.send(
                        self.client.$method(&self.route(concat!("/api/v1/", $url)))
                            .json(&form_data)
                )?;

                let status = response.status().clone();

                if status.is_client_error() {
                    return Err(Error::Client(status));
                } else if status.is_server_error() {
                    return Err(Error::Server(status));
                }

                deserialise(response)
            }
        }

        route!{$($rest)*}
    };

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            fn $name(&self) -> Result<$ret> {
                self.$method(self.route(concat!("/api/v1/", $url)))
            }
        }

        route!{$($rest)*}
    };

    () => {}
}

macro_rules! route_id {

    ($(($method:ident) $name:ident: $url:expr => $ret:ty,)*) => {
        $(
            doc_comment! {
                concat!(
                    "Equivalent to `/api/v1/",
                    $url,
                    "`\n# Errors\nIf `access_token` is not set."),
                fn $name(&self, id: u64) -> Result<$ret> {
                    self.$method(self.route(&format!(concat!("/api/v1/", $url), id)))
                }
            }
         )*
    }

}
macro_rules! paged_routes_with_id {

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            fn $name(&self, id: &str) -> Result<Page<$ret, H>> {
                let url = self.route(&format!(concat!("/api/v1/", $url), id));
                let response = self.send(
                        &mut self.client.$method(&url)
                )?;

                Page::new(self, response)
            }
        }

        route!{$($rest)*}
    };
}
