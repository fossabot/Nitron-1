def main(request, response):
    """
    Check that headers sent to navigate here *do* contain the device-memory client
    hint, and report success/failure in a way compatible with
    verify_{subresource|iframe}_state() in accept-ch-test.js
    """

    if b"device-memory" in request.headers and b"sec-ch-device-memory" in request.headers:
      result = u"PASS"
    else:
      result = u"FAIL"

    content = u'''
<script>
  let messagee = window.opener || window.parent;
  messagee.postMessage("%s" , "*");
</script>
''' % (result)
    headers = [(b"Content-Type", b"text/html"), (b"Access-Control-Allow-Origin", b"*")]
    return 200, headers, content
