$(function () {
    var formData = JSON.stringify($("#form").serializeArray());
    $('#form.submit').on
    $.ajax({
        type: "POST",
        url: "/encode",
        data: formData,
        success: function () { },
        dataType: "json",
        contentType: "application/json"
    });
});