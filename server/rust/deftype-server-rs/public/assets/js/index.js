var api = (function(){
    var ret = {
        server: {
            time: function(){
                return $.getJSON("/api/server/time");
            },
            mode: function(){
                return $.getJSON("/api/server/mode");
            }
        },
        users: {
            login: function(name, password){
                var login_form ={login_name: name, password: password};
                return $.ajax({
                    url: "/api/users/login",
                    data: JSON.stringify(login_form),
                    contentType :   'application/json',
                    method: "POST",
                    dataType: "json"
                });
            }
        }
    };

    return ret;
})();

$(function(){
    var token = new Date();
    $.ajaxSetup( {beforeSend: function(jqXHR) {
        jqXHR.setRequestHeader("Authorization", "Bearer " + token);
        console.log(token);
    } } );

   function load_server_time(){
       api.server.time().done(function(ret){
          $("#time").html("server time:" + ret.data.now);
       });
   }

   api.users.login("admin", "123456").done(function(ret){
      console.log(JSON.stringify(ret));
      if(ret.ok){
          token = ret.data.token;
          $("#msg").load("/api");

           setInterval(load_server_time, 1000);

           api.server.mode().done(function(ret){
              if(ret.data == "development"){
                $("#dev").load("/dev/1.html").show();
              }
           });
      }else{

      }
  }).fail(function( jqXHR, textStatus ) {
      alert( "Request failed: " + textStatus );
  });
});
