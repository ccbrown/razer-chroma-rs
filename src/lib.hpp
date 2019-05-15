#ifdef _WIN32
#include <windows.h>
#else
// The SDK only supports Windows. But if we define some basic types, we can still include the headers and just make Load() return false.
#include <cstddef>

#define GUID_DEFINED 1
typedef struct _GUID {
    unsigned long  Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char  Data4[ 8 ];
} GUID;

#define WM_APP 0x8000

typedef unsigned long DWORD;
typedef DWORD COLORREF;
typedef long LONG;
typedef unsigned int UINT;

static const LONG ERROR_SUCCESS = 0;
#endif

#include <RzChromaSDKDefines.h>
#include <RzChromaSDKTypes.h>
#include <RzErrors.h>

extern "C" {
	bool Load();

	RZRESULT ChromaSDKInit();
	RZRESULT ChromaSDKUnInit();
	RZRESULT ChromaSDKCreateKeyboardEffect(ChromaSDK::Keyboard::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
	RZRESULT ChromaSDKCreateHeadsetEffect(ChromaSDK::Headset::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
	RZRESULT ChromaSDKCreateMousepadEffect(ChromaSDK::Mousepad::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
	RZRESULT ChromaSDKCreateMouseEffect(ChromaSDK::Mouse::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
	RZRESULT ChromaSDKCreateKeypadEffect(ChromaSDK::Keypad::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
	RZRESULT ChromaSDKCreateChromaLinkEffect(ChromaSDK::ChromaLink::EFFECT_TYPE Effect, PRZPARAM pParam, RZEFFECTID *pEffectId);
	RZRESULT ChromaSDKSetEffect(RZEFFECTID EffectId);
	RZRESULT ChromaSDKDeleteEffect(RZEFFECTID EffectId);
}
