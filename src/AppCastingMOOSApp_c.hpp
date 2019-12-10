#ifndef MOOS_APPCASTINGMOOSAPP_C_HPP
#define MOOS_APPCASTINGMOOSAPP_C_HPP

#ifdef __cplusplus
extern "C" {
#endif

enum DataType {
    DOUBLE,
    STRING,
};

typedef struct {
    char *name;
    DataType kind;
    double d_value;
    char *s_value;
} Envelope;

typedef struct MoosApp MoosApp;

typedef bool (*rust_bool_void_star_callback)(void *callback_target);

typedef bool (*on_new_mail_callback)(void *callback_target, const Envelope *mail, unsigned int size);

MoosApp *newMoosApp();

void deleteMoosApp(MoosApp *v);

void MoosApp_setTarget(MoosApp *v, void *target);

void MoosApp_setIterateCallback(MoosApp *v, rust_bool_void_star_callback callback);

void MoosApp_setOnStartUpCallback(MoosApp *v, rust_bool_void_star_callback callback);

void MoosApp_setOnConnectToServerCallback(MoosApp *v, rust_bool_void_star_callback callback);

void MoosApp_setOnNewMailCallback(MoosApp *v, on_new_mail_callback callback);

bool MoosApp_run(MoosApp *v, const char *sName, const char *mission_file);

bool MoosApp_notifyDouble(MoosApp *v, const char *s_var, const double d_val);

bool MoosApp_notifyString(MoosApp *v, const char *s_var, const char *s_val);

bool MoosApp_register(MoosApp *v, const char *s_var, const double d_interval);

bool MoosApp_getDoubleAppConfigParam(MoosApp *v, const char *sName, double *d_var);

bool MoosApp_getStringAppConfigParam(MoosApp *v, const char *sName, char *s_var);

#ifdef __cplusplus
};
#endif
#endif //MOOS_APPCASTINGMOOSAPP_C_HPP
