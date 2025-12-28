/**
 * РЎРєСЂРёРїС‚ РґР»СЏ Р°РІС‚РѕРјР°С‚РёС‡РµСЃРєРѕРіРѕ РёСЃРїСЂР°РІР»РµРЅРёСЏ СЂР°СЃРїСЂРѕСЃС‚СЂР°РЅРµРЅРЅС‹С… РѕС€РёР±РѕРє Р»РёРЅС‚РёРЅРіР°
 */

# РСЃРїСЂР°РІР»СЏРµРј @ts-ignore РЅР° @ts-expect-error РІРѕ РІСЃРµС… С„Р°Р№Р»Р°С…
Get-ChildItem -Path src -Recurse -Include "*.ts","*.js","*.svelte" | 
  Where-Object { .FullName -notlike "*node_modules*" } | 
  ForEach-Object { 
    (Get-Content .FullName -Raw) -replace '@ts-ignore', '@ts-expect-error' | 
    Set-Content .FullName -NoNewline -Encoding UTF8
  }

# РЈРґР°Р»СЏРµРј РЅРµРёСЃРїРѕР»СЊР·СѓРµРјС‹Рµ РёРјРїРѕСЂС‚С‹ (С‚СЂРµР±СѓРµС‚ СЂСѓС‡РЅРѕР№ РїСЂРѕРІРµСЂРєРё)
Write-Host "РќР°Р№РґРµРЅС‹ СЃР»РµРґСѓСЋС‰РёРµ РЅРµРёСЃРїРѕР»СЊР·СѓРµРјС‹Рµ РїРµСЂРµРјРµРЅРЅС‹Рµ - РїСЂРѕРІРµСЂСЊС‚Рµ Рё СѓРґР°Р»РёС‚Рµ РІСЂСѓС‡РЅСѓСЋ:"
npm run lint 2>&1 | Select-String "is defined but never used" | ForEach-Object { .Line }

Write-Host "
РќР°Р№РґРµРЅС‹ @ts-ignore РєРѕРјРјРµРЅС‚Р°СЂРёРё - Р·Р°РјРµРЅРµРЅС‹ РЅР° @ts-expect-error"
