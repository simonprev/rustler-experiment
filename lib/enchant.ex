defmodule Enchant do
  use Rustler, otp_app: :enchant, crate: "enchant_elixir"

  defmodule Word do
    defstruct suggestions: [], word: nil
  end

  def check(_words, _dictionary), do: :erlang.nif_error(:nif_not_loaded)
end
