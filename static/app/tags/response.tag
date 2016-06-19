<response>
    <span id="response-{ opts.type }" class="{ css_color } hidden"><yield /></span>

    <script>
         this.css_color = "text-danger";
        if (this.opts.type == "good") {
            this.css_color = "text-success";
        }
    </script>
</response>