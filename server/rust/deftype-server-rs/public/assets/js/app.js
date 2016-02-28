var app = (function() {
    function logout() {
        sessionStorage.clear();
    }

    function setToken(token) {
        sessionStorage.token = token;
    }

    function getToken() {
        return sessionStorage.token;
    }

    function isLogined() {
        return !!(app.getToken());
    }

    return {
        setToken: setToken,
        getToken: getToken,
        isLogined: isLogined,
        logout: logout
    }
})();

var api =
    (function() {
        function json_post(url, data) {
            return $.ajax({
                url: url,
                data: JSON.stringify(data),
                contentType: 'application/json',
                method: "POST",
                dataType: "json"
            })
        }

        return {
            server: {
                time: function() {
                    return $.getJSON("/api/server/time");
                },
                mode: function() {
                    return $.getJSON("/api/server/mode");
                }
            },
            users: {
                login: function(name, password) {
                    var login_form = {
                        login_name: name,
                        password: password
                    };
                    return json_post("/api/users/login", login_form);
                },
                list: function() {
                    return $.getJSON("/api/users");
                }
            }
        };
    })();

(function() {
    $.ajaxSetup({
        beforeSend: function(jqXHR) {
            var token = app.getToken();
            jqXHR.setRequestHeader("Authorization", "Bearer " + token);
        }
    });
})();

var index = (function() {
    function init() {
        var $login = $("#login");
        if (app.isLogined()) {
            console.log("登录了");
            $login.html("已登录, <button id='btn-logout'>注销</button>");

            $("#msg").load("/api");

            function load_server_time() {
                api.server.time().done(function(ret) {
                    $("#time").html("server time:" + ret.data.now);
                });
            }

            setInterval(load_server_time, 10000);

            api.server.mode().done(function(ret) {
                if (ret.data == "development") {
                    $("#dev").load("/dev/1.html").css('visibility', 'visible');
                }
            });
        } else {
            $login.html("<a href='/login.html'>登录</a>")
        }

        $("body").on("click", "button#btn-logout", function() {
            console.log("logout");
            app.logout();
            window.location.href = "/";
        });

        $("body").on("click", "a#btn-users-list", function() {
            api.users.list().done(function(ret) {
                $("#div-users").html(JSON.stringify(ret));
            });
        });
    }

    return {
        init: init
    };
})();

var login = (function() {
    function init() {
        $('#btn-login').click(function() {
            var username = $('#username').val();
            var password = $('#password').val();

            api.users.login(username, password).done(function(ret) {
                console.log(JSON.stringify(ret));
                if (ret.ok) {
                    app.setToken(ret.data.token);
                    window.location.href = "/";
                } else {
                    alert("登录失败!");
                }
            }).fail(function(jqXHR, textStatus) {
                alert("Request failed: " + textStatus);
            });
        });
    }

    return {
        init: init
    };
})();
