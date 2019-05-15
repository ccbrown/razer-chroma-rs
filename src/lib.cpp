#include "lib.hpp"

#ifdef _WIN32
#include <Tchar.h>

#ifdef _WIN64
#define CHROMASDKDLL _T("RzChromaSDK64.dll")
#else
#define CHROMASDKDLL _T("RzChromaSDK.dll")
#endif
#endif

typedef RZRESULT (*INIT)(void);
typedef RZRESULT (*UNINIT)(void);
typedef RZRESULT (*CREATEKEYBOARDEFFECT)(ChromaSDK::Keyboard::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
typedef RZRESULT (*CREATEHEADSETEFFECT)(ChromaSDK::Headset::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
typedef RZRESULT (*CREATEMOUSEPADEFFECT)(ChromaSDK::Mousepad::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
typedef RZRESULT (*CREATEMOUSEEFFECT)(ChromaSDK::Mouse::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
typedef RZRESULT (*CREATEKEYPADEFFECT)(ChromaSDK::Keypad::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
typedef RZRESULT (*CREATECHROMALINKEFFECT)(ChromaSDK::ChromaLink::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
typedef RZRESULT (*SETEFFECT)(RZEFFECTID EffectId);
typedef RZRESULT (*DELETEEFFECT)(RZEFFECTID EffectId);

INIT gInit = nullptr;
UNINIT gUnInit = nullptr;
CREATEKEYBOARDEFFECT gCreateKeyboardEffect = nullptr;
CREATEMOUSEEFFECT gCreateMouseEffect = nullptr;
CREATEHEADSETEFFECT gCreateHeadsetEffect = nullptr;
CREATEMOUSEPADEFFECT gCreateMousepadEffect = nullptr;
CREATEKEYPADEFFECT gCreateKeypadEffect = nullptr;
CREATECHROMALINKEFFECT gCreateChromaLinkEffect = nullptr;
SETEFFECT gSetEffect = nullptr;
DELETEEFFECT gDeleteEffect = nullptr;

#ifdef _WIN32
HMODULE gSDKModule = nullptr;
#endif

extern "C" {
	bool Load() {
        #ifdef _WIN32
            if(gSDKModule == nullptr) {
                gSDKModule = LoadLibrary(CHROMASDKDLL);
            }

            if (gSDKModule == nullptr) {
                return false;
            }

            if (gInit == nullptr) {
                gInit = (INIT)::GetProcAddress(gSDKModule, "Init");
                gUnInit = (INIT)::GetProcAddress(gSDKModule, "UnInit");
                gCreateKeyboardEffect = (CREATEKEYBOARDEFFECT)::GetProcAddress(gSDKModule, "CreateKeyboardEffect");
                gCreateMouseEffect = (CREATEMOUSEEFFECT)::GetProcAddress(gSDKModule, "CreateMouseEffect");
                gCreateMousepadEffect = (CREATEMOUSEPADEFFECT)::GetProcAddress(gSDKModule, "CreateMousepadEffect");
                gCreateKeypadEffect = (CREATEKEYPADEFFECT)::GetProcAddress(gSDKModule, "CreateKeypadEffect");
                gCreateHeadsetEffect = (CREATEHEADSETEFFECT)::GetProcAddress(gSDKModule, "CreateHeadsetEffect");
                gCreateChromaLinkEffect = (CREATECHROMALINKEFFECT)::GetProcAddress(gSDKModule, "CreateChromaLinkEffect");
                gSetEffect = (SETEFFECT)GetProcAddress(gSDKModule, "SetEffect");
                gDeleteEffect = (DELETEEFFECT)GetProcAddress(gSDKModule, "DeleteEffect");
            }
        #endif

		return gInit != nullptr;
	}

	RZRESULT ChromaSDKInit() {
		return gInit();
	}

	RZRESULT ChromaSDKUnInit() {
		return gUnInit();
	}

	RZRESULT ChromaSDKCreateKeyboardEffect(ChromaSDK::Keyboard::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId) {
		return gCreateKeyboardEffect(Effect, pParam, pEffectId);
	}

	RZRESULT ChromaSDKCreateHeadsetEffect(ChromaSDK::Headset::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId) {
		return gCreateHeadsetEffect(Effect, pParam, pEffectId);
	}

	RZRESULT ChromaSDKCreateMousepadEffect(ChromaSDK::Mousepad::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId) {
		return gCreateMousepadEffect(Effect, pParam, pEffectId);
	}

	RZRESULT ChromaSDKCreateMouseEffect(ChromaSDK::Mouse::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId) {
		return gCreateMouseEffect(Effect, pParam, pEffectId);
	}

	RZRESULT ChromaSDKCreateKeypadEffect(ChromaSDK::Keypad::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId) {
		return gCreateKeypadEffect(Effect, pParam, pEffectId);
	}

	RZRESULT ChromaSDKCreateChromaLinkEffect(ChromaSDK::ChromaLink::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId) {
		return gCreateChromaLinkEffect(Effect, pParam, pEffectId);
	}

	RZRESULT ChromaSDKSetEffect(RZEFFECTID EffectId) {
		return gSetEffect(EffectId);
	}

	RZRESULT ChromaSDKDeleteEffect(RZEFFECTID EffectId) {
		return gDeleteEffect(EffectId);
	}
}
