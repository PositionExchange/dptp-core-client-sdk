#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

void wire_initialize_orders(int64_t port_,
                            struct wire_uint_8_list *asks,
                            struct wire_uint_8_list *bids);

void wire_new_pair_order_compute(int64_t port_,
                                 struct wire_uint_8_list *pair_symbol,
                                 struct wire_uint_8_list *collateral_long_token,
                                 struct wire_uint_8_list *collateral_short_token,
                                 struct wire_uint_8_list *leverage,
                                 struct wire_uint_8_list *max_notional,
                                 struct wire_uint_8_list *min_quantity_base,
                                 struct wire_uint_8_list *margin_ratio,
                                 struct wire_uint_8_list *taker_fee,
                                 struct wire_uint_8_list *maker_fee,
                                 uint32_t base_token_precision);

void wire_change_leverage(int64_t port_,
                          struct wire_uint_8_list *new_leverage,
                          struct wire_uint_8_list *max_notional);

void wire_get_active_pair(int64_t port_);

void wire_change_active_pair(int64_t port_, struct wire_uint_8_list *new_active_pair);

void wire_check_pair_exists(int64_t port_, struct wire_uint_8_list *new_active_pair);

void wire_update_balance(int64_t port_,
                         struct wire_uint_8_list *token,
                         struct wire_uint_8_list *balance);

void wire_compute_open_order(int64_t port_,
                             struct wire_uint_8_list *pay_token,
                             struct wire_uint_8_list *pay_amount,
                             struct wire_uint_8_list *limit_price,
                             struct wire_uint_8_list *quantity,
                             bool is_quote,
                             bool is_buy,
                             bool use_percentage);

void wire_get_order_book_manager(int64_t port_);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_initialize_orders);
    dummy_var ^= ((int64_t) (void*) wire_new_pair_order_compute);
    dummy_var ^= ((int64_t) (void*) wire_change_leverage);
    dummy_var ^= ((int64_t) (void*) wire_get_active_pair);
    dummy_var ^= ((int64_t) (void*) wire_change_active_pair);
    dummy_var ^= ((int64_t) (void*) wire_check_pair_exists);
    dummy_var ^= ((int64_t) (void*) wire_update_balance);
    dummy_var ^= ((int64_t) (void*) wire_compute_open_order);
    dummy_var ^= ((int64_t) (void*) wire_get_order_book_manager);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}