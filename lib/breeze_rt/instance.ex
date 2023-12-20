defmodule BreezeRt.Instance do
  @moduledoc false

  use ThousandIsland.Handler

  @impl ThousandIsland.Handler
  def handle_connection(_socket, _state) do
    channel = BreezeRt.SocketMan.start(self())

    {:continue, channel}
  end

  @impl ThousandIsland.Handler
  def handle_data(data, _socket, state) do
    :ok = BreezeRt.SocketMan.send_message(state, data)

    {:continue, state}
  end

  @impl GenServer
  def handle_info({:message_write, binary}, {socket, state}) do
    ThousandIsland.Socket.send(socket, binary)

    {:noreply, {socket, state}, socket.read_timeout}
  end

  def handle_info(:message_flush, {socket, state}) do
    # do nothing

    {:noreply, {socket, state}, socket.read_timeout}
  end

  def handle_info(:message_shutdown, {socket, state}) do
    ThousandIsland.Socket.close(socket)

    {:noreply, {socket, state}, socket.read_timeout}
  end
end
