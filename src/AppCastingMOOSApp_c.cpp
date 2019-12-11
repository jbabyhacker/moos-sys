#include "AppCastingMOOSApp_c.hpp"
#include "MOOS/libMOOS/Thirdparty/AppCasting/AppCastingMOOSApp.h"
#include <set>

class MoosApp : public AppCastingMOOSApp {
public:
    using AppCastingMOOSApp::Notify;
    using AppCastingMOOSApp::RegisterVariables;
    using AppCastingMOOSApp::Register;
    using AppCastingMOOSApp::m_MissionReader;

    void *m_callbackTarget = nullptr;
    rust_bool_void_star_callback m_iterateCallback = nullptr;
    rust_bool_void_star_callback m_onStartUpCallback = nullptr;
    rust_bool_void_star_callback m_onConnectToServerCallback = nullptr;
    on_new_mail_callback m_onNewMailCallback = nullptr;

protected:
    bool Iterate() override {
        bool success = false;
        AppCastingMOOSApp::Iterate();
        if (m_iterateCallback) {
            success = m_iterateCallback(m_callbackTarget);
        }
        AppCastingMOOSApp::PostReport();
        return success;
    }

    bool OnNewMail(MOOSMSG_LIST &NewMail) override {
        AppCastingMOOSApp::OnNewMail(NewMail);

        MOOSMSG_LIST::iterator p;
        Envelope mail[NewMail.size()];
        std::size_t index = 0;
        for (p = NewMail.begin(); p != NewMail.end();) {
            CMOOSMsg &msg = *p;
            if (msg.IsDouble()) {
                mail[index] = {strdup(msg.GetKey().c_str()), DOUBLE, msg.GetDouble(), nullptr};
                index += 1;
                p = NewMail.erase(p);
            } else if (msg.IsString()) {
                mail[index] = {strdup(msg.GetKey().c_str()), STRING, 0,
                               strdup(msg.GetString().c_str())};
                index += 1;
                p = NewMail.erase(p);
            } else {
                ++p;
            }
        }

        if (m_onNewMailCallback) {
            return m_onNewMailCallback(m_callbackTarget, mail, index);
        }

        return false;
    }

    bool OnStartUp() override {
        AppCastingMOOSApp::OnStartUp();
        if (m_onStartUpCallback) {
            return m_onStartUpCallback(m_callbackTarget);
        } else {
            return false;
        }
    }

    bool OnConnectToServer() override {
        if (m_onConnectToServerCallback) {
            RegisterVariables();
            return m_onConnectToServerCallback(m_callbackTarget);
        }

        return false;
    }

private:

};

extern "C" {
MoosApp *newMoosApp() {
    return new MoosApp();
}

void deleteMoosApp(MoosApp *v) {
    delete v;
}

void MoosApp_setTarget(MoosApp *v, void *target) {
    v->m_callbackTarget = target;
}

void MoosApp_setIterateCallback(MoosApp *v, rust_bool_void_star_callback callback) {
    v->m_iterateCallback = callback;
}

void MoosApp_setOnStartUpCallback(MoosApp *v, rust_bool_void_star_callback callback) {
    v->m_onStartUpCallback = callback;
}

void MoosApp_setOnConnectToServerCallback(MoosApp *v, rust_bool_void_star_callback callback) {
    v->m_onConnectToServerCallback = callback;
}

void MoosApp_setOnNewMailCallback(MoosApp *v, on_new_mail_callback callback) {
    v->m_onNewMailCallback = callback;
}

bool MoosApp_notifyDouble(MoosApp *v, const char *s_var, const double d_val) {
    return v->Notify(s_var, d_val);
}

bool MoosApp_notifyString(MoosApp *v, const char *s_var, const char *s_val) {
    return v->Notify(s_var, s_val);
}

bool MoosApp_run(MoosApp *v, const char *sName, const char *mission_file) {
    std::string cppName(sName);
    std::string cppMissionFile(mission_file);

    return v->Run(cppName, cppMissionFile);
}

bool MoosApp_register(MoosApp *v, const char *s_var, const double d_interval) {
    std::string cppString(s_var);

    return v->Register(cppString, d_interval);
}

bool MoosApp_getDoubleGlobalConfigParam(MoosApp *v, const char *sName, double *d_var) {
    std::string cppName(sName);
    std::string cppValue;

    return v->m_MissionReader.GetValue(cppName, *d_var);
}

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-but-set-parameter"
bool MoosApp_getStringGlobalConfigParam(MoosApp *v, const char *sName, char *s_var) {
#pragma GCC diagnostic pop

    std::string cppName(sName);
    std::string cppValue;

    bool result = v->m_MissionReader.GetValue(cppName, cppValue);
    s_var = strdup(const_cast<char*>(cppValue.c_str()));

    return result;
}

bool MoosApp_getDoubleAppConfigParam(MoosApp *v, const char *sName, double *d_var) {
    std::string cppName(sName);
    std::string cppValue;

    return v->m_MissionReader.GetConfigurationParam(cppName, *d_var);
}

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-but-set-parameter"
bool MoosApp_getStringAppConfigParam(MoosApp *v, const char *sName, char *s_var) {
#pragma GCC diagnostic pop

    std::string cppName(sName);
    std::string cppValue;

    bool result = v->m_MissionReader.GetConfigurationParam(cppName, cppValue);
    s_var = strdup(const_cast<char*>(cppValue.c_str()));

    return result;
}

}
