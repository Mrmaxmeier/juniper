(function() {var implementors = {};
implementors["iron"] = [];implementors["juniper"] = ["impl&lt;CtxFactory,&nbsp;Query,&nbsp;Mutation,&nbsp;CtxT&gt; <a class='trait' href='iron/middleware/trait.Handler.html' title='iron::middleware::Handler'>Handler</a> for <a class='struct' href='juniper/iron_handlers/struct.GraphQLHandler.html' title='juniper::iron_handlers::GraphQLHandler'>GraphQLHandler</a>&lt;CtxFactory,&nbsp;Query,&nbsp;Mutation,&nbsp;CtxT&gt; <span class='where'>where CtxFactory: <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Fn.html' title='core::ops::Fn'>Fn</a>(&amp;mut <a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>) -&gt; CtxT + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a> + 'static, CtxT: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a> + 'static, Query: <a class='trait' href='juniper/trait.GraphQLType.html' title='juniper::GraphQLType'>GraphQLType</a>&lt;CtxT&gt; + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a> + 'static, Mutation: <a class='trait' href='juniper/trait.GraphQLType.html' title='juniper::GraphQLType'>GraphQLType</a>&lt;CtxT&gt; + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a> + 'static</span>","impl <a class='trait' href='iron/middleware/trait.Handler.html' title='iron::middleware::Handler'>Handler</a> for <a class='struct' href='juniper/iron_handlers/struct.GraphiQLHandler.html' title='juniper::iron_handlers::GraphiQLHandler'>GraphiQLHandler</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()