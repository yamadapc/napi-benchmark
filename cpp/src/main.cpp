#include <napi.h>

static void Hello(const Napi::CallbackInfo& info) {
  return;
}

static Napi::Value RoundTrip(const Napi::CallbackInfo& info) {
  return info[0];
}

static Napi::Object Init(Napi::Env env, Napi::Object exports) {
  exports.Set(Napi::String::New(env, "hello"),
              Napi::Function::New(env, Hello));
  exports.Set(Napi::String::New(env, "roundTrip"),
              Napi::Function::New(env, RoundTrip));
  return exports;
}

NODE_API_MODULE(benchmark_addon, Init)
