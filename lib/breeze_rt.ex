defmodule BreezeRt do
  def serve(
        binary \\ "HEAD / HTTP/1.1\r\nHost: localhost:2137\r\nUser-Agent: curl/8.1.2\r\nAccept: */*\r\n\r\n"
      ) do
    channel = BreezeRt.SocketMan.start(self())

    :ok = BreezeRt.SocketMan.send_message(channel, binary)
  end
end
