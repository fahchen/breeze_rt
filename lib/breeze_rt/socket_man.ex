defmodule BreezeRt.SocketMan do
  use Rustler, otp_app: :breeze_rt, crate: "breezert_socketman"

  @typep channel() :: reference()

  @spec start(pid()) :: channel()
  def start(_pid), do: error()

  @spec send_message(channel(), binary()) :: :ok
  def send_message(_channel, _bytes), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
