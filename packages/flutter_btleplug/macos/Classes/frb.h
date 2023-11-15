#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_Characteristic {
  const void *ptr;
} wire_Characteristic;

typedef struct wire_BleCharacteristic {
  struct wire_Characteristic characteristic;
} wire_BleCharacteristic;

typedef struct DartCObject *WireSyncReturn;

jint JNI_OnLoad(JavaVM vm, const void *res);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_init(int64_t port_);

void wire_scan(int64_t port_, struct wire_StringList *filter);

void wire_events(int64_t port_);

void wire_connect(int64_t port_, struct wire_uint_8_list *id);

void wire_disconnect(int64_t port_, struct wire_uint_8_list *id);

void wire_discover_services(int64_t port_, struct wire_uint_8_list *id);

void wire_create_log_stream(int64_t port_);

void wire_uuid__method__BleCharacteristic(int64_t port_, struct wire_BleCharacteristic *that);

void wire_service_uuid__method__BleCharacteristic(int64_t port_,
                                                  struct wire_BleCharacteristic *that);

void wire_properties__method__BleCharacteristic(int64_t port_, struct wire_BleCharacteristic *that);

void wire_descriptors__method__BleCharacteristic(int64_t port_,
                                                 struct wire_BleCharacteristic *that);

struct wire_Characteristic new_Characteristic(void);

struct wire_StringList *new_StringList_0(int32_t len);

struct wire_BleCharacteristic *new_box_autoadd_ble_characteristic_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void drop_opaque_Characteristic(const void *ptr);

const void *share_opaque_Characteristic(const void *ptr);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_init);
    dummy_var ^= ((int64_t) (void*) wire_scan);
    dummy_var ^= ((int64_t) (void*) wire_events);
    dummy_var ^= ((int64_t) (void*) wire_connect);
    dummy_var ^= ((int64_t) (void*) wire_disconnect);
    dummy_var ^= ((int64_t) (void*) wire_discover_services);
    dummy_var ^= ((int64_t) (void*) wire_create_log_stream);
    dummy_var ^= ((int64_t) (void*) wire_uuid__method__BleCharacteristic);
    dummy_var ^= ((int64_t) (void*) wire_service_uuid__method__BleCharacteristic);
    dummy_var ^= ((int64_t) (void*) wire_properties__method__BleCharacteristic);
    dummy_var ^= ((int64_t) (void*) wire_descriptors__method__BleCharacteristic);
    dummy_var ^= ((int64_t) (void*) new_Characteristic);
    dummy_var ^= ((int64_t) (void*) new_StringList_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_ble_characteristic_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) drop_opaque_Characteristic);
    dummy_var ^= ((int64_t) (void*) share_opaque_Characteristic);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
