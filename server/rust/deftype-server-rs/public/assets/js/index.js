var api = (function(){
    var ret = {
        server: {
            time: function(){
                return $.getJSON("/api/server/time");
            },
            mode: function(){
                return $.getJSON("/api/server/mode");
            }
        }
    };

    return ret;
})();

$(function(){
   $("#msg").load("/api");

   function load_server_time(){
       api.server.time().done(function(ret){
          $("#time").html("server time:" + ret.now);
       });
   }

   setInterval(load_server_time, 1000);

   api.server.mode().done(function(ret){
      if(ret== "development"){
        $("#dev").load("/dev/1.html").show();
      }
   });
});
