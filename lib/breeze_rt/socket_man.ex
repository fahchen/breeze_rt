defmodule BreezeRt.SocketMan do
  use Rustler, otp_app: :breeze_rt, crate: "breezert_socketman"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
