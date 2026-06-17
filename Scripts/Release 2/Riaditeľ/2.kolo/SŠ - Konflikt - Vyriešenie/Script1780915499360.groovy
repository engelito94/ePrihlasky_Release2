import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import java.text.Normalizer as Normalizer
import dev.failsafe.internal.util.Assert as Assert
import internal.GlobalVariable as GlobalVariable
import portal.Helper as Helper
import portal.Prihlasovanie as Prihlasovanie
import org.openqa.selenium.Keys as Keys

Mail mail = new Mail()

Helper help = new Helper()

Prihlasovanie prihlasovanie = new Prihlasovanie()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021624')

WebUI.selectOptionByLabel(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/2kolo/Select_2kolo'), '2.kolo', 
    false)

'Usporiadanie prihlášok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/button_(nepovinn)_btn-zoradit-podla-predvolene'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/input_Poda priezviska (abecedne - vzostupne_a91dc1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/button_Sp_btn-zoradit govuk-button govuk-bu_4b9abc'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/button_Detail_govuk-button govuk-button--se_40abef'))

not_run: WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/span_warning_panel-text-red konflikt-ss'), 
    'Táto prihláška je v stave - V konflikte.\nPre toto dieťa existuje v systéme viacero prihlášok.Vyzvite zákonného zástupcu na výber jednej verzie.Následne vyriešte konflikt označením jednej prihlášky ako aktívnej.Bez vyriešenia konfliktu nie je možné prihlášku ďalej spracovávať.')

not_run: WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_warning_govuk-panel__body'), 
    'Táto prihláška je v stave - V konflikte.\nPre toto dieťa existuje v systéme viacero prihlášok.Vyzvite zákonného zástupcu na výber jednej verzie.Následne vyriešte konflikt označením jednej prihlášky ako aktívnej.Bez vyriešenia konfliktu nie je možné prihlášku ďalej spracovávať.')

WebUI.waitForJQueryLoad(100)

String bannerKonflikt = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/span_warning_panel-text-red konflikt-ss'))

bannerKonflikt = bannerKonflikt.replaceAll('\\r?\\n+', ' ').replaceAll('\\s+', ' ').trim()

//println(bannerKonflikt)
assert bannerKonflikt.equals('Táto prihláška je v stave - V konflikte. Pre toto dieťa existuje v systéme viacero prihlášok. Vyzvite zákonného zástupcu na výber jednej verzie. Následne vyriešte konflikt označením jednej prihlášky ako aktívnej. Bez vyriešenia konfliktu nie je možné prihlášku ďalej spracovávať.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Zrui prihlku_panel-header red'), 
    'Duplicitné prihlášky')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge red'), 
    'V konflikte')

String meno = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Meno_dietaMeno'))

String priezvisko = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'))

String identifikator = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Identifiktor prihlky_prihlaskaIdentifikator'))

String datumNarodenia = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/DatumNrodeniaZiaka'))

'Výzva na vyriešenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/button_Oznai ako skontrolovan_btn-vyzva-na-_87c62a'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Vyriei konflikt_vyzva-riesenie-konfliktu-title'), 
    'Výzva na riešenie konfliktu')

String text = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Vzva bude zaslan nasledujcim osobm_vyzv_265567'))

assert (text.equals("Tomáš Lukáč") || text.equals("Uršula Zálesná"))
	
text = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Dtum podania_vyzva-zz-item vyzva-zz-men_eddbd7'))

assert (text.equals("Tomáš Lukáč") || text.equals("Uršula Zálesná"))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Polia oznaen hviezdikou s povinn_infoTe_f289d0'), 
    'Pre dieťa existuje viac ako jedna prihláška. Vyzvite zákonných zástupcov k výberu jednej prihlášky prostredníctvom emailovej notifikácie.')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/textarea_(nepovinn)_textarea-sprievodnaSpra_6c0013'), 
    'Prosím o vyriešenie konfliktu. Váš katalon :)')

'Odoslať výzvu na vyriešenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/button_Zrui_btn-odoslat-vyzvu govuk-button _13b04c'))

//Kontrola výzvy
String teloMailu = mail.getLastEmailText('pop.gmail.com', 'pop3', GlobalVariable.mailLogin, GlobalVariable.mailHeslo)

teloMailu = help.cleanupCidUrls(teloMailu)

teloMailu = teloMailu.replaceAll('\\r?\\n+', ' ').replaceAll('\\s+', ' ').trim()

assert teloMailu.equals(((((('Vážený/á pán/pani Tomáš Lukáč, v systéme bolo zistené, že pre žiaka ' + meno) + ' ') + priezvisko) + 
    ' nar. ') + datumNarodenia) + ' boli podané viaceré prihlášky. Riaditeľ školy Stredná škola pre AT vás týmto vyzýva, aby ste ho bezodkladne kontaktovali a informovali, ktorú prihlášku si želáte ponechať ako platnú. Sprievodná správa od riaditeľa: Prosím o vyriešenie konfliktu. Váš katalon :) Bez vyriešenia tohto konfliktu nebudú prihlášky ďalej spracované. S pozdravom Tím elektronických prihlášok MŠVVaM SR Tento email bol generovaný automaticky portálom Elektronické prihlášky do škôl, ktorý je v správe Ministerstva školstva, výskumu, vývoja a mládeže Slovenskej republiky. Neodpovedajte naň.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/span_R_sprava-nazov'), 
    'Riaditeľ školy Stredná škola pre AT zaslal výzvu na riešenie konfliktu prihlášok.')

'Vyriešenie konfliktu riaditeľom'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/button_Vyzva na rieenie konfliktu_btn-vyrie_bcc726'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Zrui prihlku_vyriesit-konflikt-title'), 
    'Vyriešiť konflikt')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Vyriei konflikt_infoTextBoxHeader'), 
    'Po označení prihlášky ako aktívnej sa automaticky zneaktivnia všetky duplicitné prihlášky pre toto dieťa na všetkých školách.')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/textarea_(nepovinn)_textarea-sprievodnaSpra_ccada5'), 
    'Konflikt vyriešený.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/button_Sp_btn-vyriesit-konflikt govuk-butto_2b9a96'))

int lastDash = identifikator.lastIndexOf('-')

String prefix = identifikator[(0..lastDash) // "P-2026-"
]

String numberPart = identifikator[(lastDash + 1..-1) // "13334"
]

int lastNumber = numberPart.toInteger()

String identifikator1 = prefix+(lastNumber + 1).toString()

//Kontrola vyriešenia
teloMailu = mail.getLastEmailText('pop.gmail.com', 'pop3', GlobalVariable.mailLogin, GlobalVariable.mailHeslo)

teloMailu = help.cleanupCidUrls(teloMailu)

teloMailu = teloMailu.replaceAll('\\r?\\n+', ' ').replaceAll('\\s+', ' ').trim()

assert teloMailu.equals(((((((('Vážený/á pán/pani Tomáš Lukáč, Prihláška ' + identifikator) + ' bola v konflikte s prihláškou/prihláškami ') + 
    identifikator) + ', ') + identifikator1) + '. Konflikt bol vyriešený . V systéme bude ďalej evidovaná len prihláška ') + 
    identifikator) + '. S pozdravom Tím elektronických prihlášok MŠVVaM SR Tento email bol generovaný automaticky portálom Elektronické prihlášky do škôl, ktorý je v správe Ministerstva školstva, výskumu, vývoja a mládeže Slovenskej republiky. Neodpovedajte naň.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_info_panel-text-blue-700 prihlaska-aktivna'), 
    'Prihláška bola označená ako aktívna riaditeľom školy 910021624 Stredná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Prihlka bola oznaen ako aktvna riaditeo_fcdf53'), 
    'Ak je potrebné označiť inú prihlášku ako aktívnu, kontaktujte riaditeľa uvedenej školy.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/strong_1'), 
    'Duplicita prihlášok úspešne vyriešená')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge'), 
    'V spracovaní')

/**
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_warning_panel-text-red-700 prihlaska-zn_eea6fc'), 
    'Prihláška bola zneaktívnená riaditeľom školy 910021624 Stredná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Prihlka bola zneaktvnen riaditeom koly _831a43'), 
    'Zmeniť výber aktívnej prihlášky môže len riaditeľ, ktorý takto prihlášku označil.')
**/
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/SpravaPrihlasokMenu'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    (meno.toString() + ' ') + priezvisko.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/button_Vyhadvanie v prihlkach_fulltext-inpu_1e6782'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/div_Identifiktor_data-prihlaska-stav badge green'), 
    'V spracovaní')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/div_Elektronicky_grey-label'), 
    identifikator.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/div_P-2026-13313_data-prihlaska-stav badge red'), 
    'Konflikt - neaktívny')

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/div_Papierovo_grey-label'), 'P-2026-13314')
prihlasovanie.odhlasPouzivatela()

