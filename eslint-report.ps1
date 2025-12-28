/**
 * РЎРѕР·РґР°СЋ С„РёРЅР°Р»СЊРЅС‹Р№ РѕС‚С‡РµС‚ РѕР± РёСЃРїСЂР°РІР»РµРЅРёРё ESLint
 */

Write-Host "=== РћРўР§Р•Рў РћР‘ РРЎРџР РђР’Р›Р•РќРР ESLINT ===" -ForegroundColor Green
Write-Host "РСЃРїРѕР»СЊР·РѕРІР°РЅ Context7 РґР»СЏ РїРѕР»СѓС‡РµРЅРёСЏ РѕС„РёС†РёР°Р»СЊРЅРѕР№ РґРѕРєСѓРјРµРЅС‚Р°С†РёРё Svelte ESLint" -ForegroundColor Cyan
Write-Host ""

# РџСЂРѕРІРµСЂСЏРµРј С‚РµРєСѓС‰СѓСЋ РєРѕРЅС„РёРіСѓСЂР°С†РёСЋ
Write-Host "РўРµРєСѓС‰Р°СЏ РєРѕРЅС„РёРіСѓСЂР°С†РёСЏ ESLint:" -ForegroundColor Yellow
Get-Content eslint.config.js | Select-Object -First 10

Write-Host "
РСЃРїСЂР°РІР»РµРЅРЅС‹Рµ РїСЂРѕР±Р»РµРјС‹:" -ForegroundColor Green
Write-Host "вњ“ Р—Р°РјРµРЅРµРЅ СЃС‚Р°СЂС‹Р№ @ota-meshi/eslint-plugin-svelte РЅР° РѕС„РёС†РёР°Р»СЊРЅС‹Р№ eslint-plugin-svelte"
Write-Host "вњ“ Р”РѕР±Р°РІР»РµРЅ typescript-eslint РґР»СЏ TypeScript РїРѕРґРґРµСЂР¶РєРё"
Write-Host "вњ“ РќР°СЃС‚СЂРѕРµРЅС‹ РіР»РѕР±Р°Р»СЊРЅС‹Рµ РїРµСЂРµРјРµРЅРЅС‹Рµ РґР»СЏ Svelte 5 runes"
Write-Host "вњ“ РРЅС‚РµРіСЂРёСЂРѕРІР°РЅР° svelte.config.js РґР»СЏ РїР°СЂСЃРµСЂР°"
Write-Host "вњ“ РЈР»СѓС‡С€РµРЅР° РєРѕРЅС„РёРіСѓСЂР°С†РёСЏ РґР»СЏ .svelte С„Р°Р№Р»РѕРІ"

# Р¤РёРЅР°Р»СЊРЅР°СЏ РїСЂРѕРІРµСЂРєР°
Write-Host "
Р—Р°РїСѓСЃРє С„РёРЅР°Р»СЊРЅРѕРіРѕ С‚РµСЃС‚Р° Р»РёРЅС‚РµСЂР°..." -ForegroundColor Yellow
npm run lint 2>&1 | Select-Object -Last 1
