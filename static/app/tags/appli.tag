<appli>
    <div id="page-wrapper" style="margin-left: 0;">
        <div class="row">
            <div class="col-md-12">
                <yield/>
            </div>
        </div>
    </div>
    <script>
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
    </script>
</appli>