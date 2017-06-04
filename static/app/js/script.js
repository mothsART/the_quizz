var questions = new Vue({
  delimiters: ['{', '}'],
  el: "#questions",
  data: {
    question: null
  },
  created: function () {
    axios.get(
      '/question'
    ).then(function(response) {
      questions.question = response.data;
    });
  },
});

var buttons = new Vue({
  el: "#buttons",
  delimiters: ['{', '}'],
  data: {
  },
  methods: {
    reply: function(choice) {
      "use strict";
      var buffer = msgpack.encode(0);
      axios.post(
        '/reply', buffer,
        {
          responseType: 'blob',
          headers: {'Content-Type': 'application/msgpack; charset=utf-8'}
        }
      ).then(function(response) {
        var reader = new FileReader();
        reader.onload = function (e) {
            var value = msgpack.decode(new Uint8Array(reader.result));
            if (value == choice)
            {
              resultVue.cssColorChecked   = "green-text";
              resultVue.cssColorUnchecked = "hidden";
            }
            else {
              resultVue.cssColorChecked = "hidden";
              resultVue.cssColorUnchecked = "red-text";
            }
        }
        reader.readAsArrayBuffer(response.data);
      });
    }
  }
});

var resultVue = new Vue({
  el: "#result",
  delimiters: ['{', '}'],
  data: {
    cssColorUnchecked : "hidden",
    cssColorChecked   : "hidden"
  },
  methods: {
    create: function(event) {
      "use strict";
      //this.display_result = "hidden";
    }
  }
});

/*
        var questions_tag = this.tags.questions;
        this.on('before-mount', function() {
            $.ajax({
                method: "POST",
                dataType: "json",
                url: '/i18n_strings'
            }).done(function(success_message) {
                $("#title").text(success_message.title);
                $("#response-good").text(success_message.checked);
                $("#response-bad").text(success_message.unchecked);
                 $("#yes-button").attr("value", success_message.yes);
                $("#no-button").attr("value", success_message.no);
                $.ajax({
                    method: "POST",
                    dataType: "json",
                    url: '/question'
                }).done(function(success_message) {
                    questions_tag.indice = success_message.indice;
                    questions_tag.question = success_message.entitled;
                    questions_tag.update();
                });
            });
        });

        choice(e) {
            $.ajax({
                method: "POST",
                dataType: "json",
                url: '/reply',
                data: JSON.stringify({
                    indice: questions_tag.indice,
                    choice: e.srcElement.value
                })
            })
            .done(function(success_message) {
                if (success_message.response) {
                    $("#response-good").removeClass("hidden");
                    $("#response-bad").addClass("hidden");
                }
                else {
                    $("#response-bad").removeClass("hidden");
                    $("#response-good").addClass("hidden");
                }
            });
        };
*/