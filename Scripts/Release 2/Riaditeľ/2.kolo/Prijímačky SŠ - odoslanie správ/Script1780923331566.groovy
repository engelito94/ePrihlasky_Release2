import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import java.time.LocalDate
import java.time.LocalDateTime

import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
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
import internal.GlobalVariable as GlobalVariable
import portal.Helper as Helper
import portal.Prihlasovanie as Prihlasovanie
import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

Mail mail = new Mail()

Helper help = new Helper()

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'
def pozvanka = filePath + '/Data Files/PozvánkaPredloha2kolo.pdf'
def body = filePath + '/Data Files/BodyPredloha.pdf'

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021624')

WebUI.selectOptionByLabel(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/2kolo/Select_2kolo'), '2.kolo',
	false)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/button_(nepovinn)_btn-zoradit-podla-predvolene'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/input_Poda priezviska (abecedne - vzostupne_a91dc1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/button_Sp_btn-zoradit govuk-button govuk-bu_4b9abc'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Prihlky a rozhodnutia  ePrihlky/button_Detail_govuk-button govuk-button--se_40abef'))

WebUI.waitForJQueryLoad(60)

String meno = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Meno_dietaMeno'))

String priezvisko = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/Page_Podrobnosti prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'))

String pristupovyKod = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/pristupovyKod'))

String datumNarodenia = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Konflikty/DatumNrodeniaZiaka'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/menuSpravaPrihlasok'))

'Prijímačky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/a_Prihlky_govuk-header__link'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_(nepovinn)_prijimacky-btn-zoradit-po_1f8ad8'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/input_Poda priezviska (abecedne - vzostupne_13afa1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Sp_btn-zoradit govuk-button govuk-bu_4b9abc'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Akcia_govuk-button govuk-button--sec_5dbbfe'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/a'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_datumDen'), 
    '25')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_datumMesiac'), 
    '9')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_datumRok'), 
    '2026')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_casHodina'), 
    '11')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_casMinuta'), 
    '11')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_miestnost'), 
    '6.C')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-edit govuk-button govuk-but_6cb5b0'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Nhradn termn_grey-label'), 
    '25.09.2026')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Nhradn termn_grey-label_1'), 
    '11:11')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Nhradn termn_grey-label_2'), 
    '6.C')

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Nhradn termn_data-komunikacia-neodoslana badge'), 'Neodoslaná')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/span_Meno, odbor a kd odboru_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Nastavenie termnu skky_vykonat-akciu-btn'))

'Pozvánka'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/a_1'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Apply_title'), 
    'Vygenerovať pozvánky')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prijimackyPredmetSpravy'), 
    'Pozvánka na prijímacie skúšky AT')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/textarea_(nepovinn)_textarea-prijimackyVoli_879f89'), 
    'Prineste si gaštany, budú sa stavať zvieratká.')

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b'), 
    priloha)

'Odoslanie pozvánky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-odoslat govuk-button govuk-_541f94'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Vygenerova pozvnky  ePrihlky/div_Vygenerova pozvnky_idsk-card__heading'), 
    'Generovanie 1 pozvánok bolo spustené.')

String teloMailu = mail.getLastEmailText('pop.gmail.com', 'pop3', GlobalVariable.mailLogin, GlobalVariable.mailHeslo)

teloMailu = help.cleanupCidUrls(teloMailu)

teloMailu = teloMailu.replaceAll('\\r?\\n+', ' ').replaceAll('\\s+', ' ').trim()

println(teloMailu)

assert teloMailu.contains(((((((('Vážený/á pán/pani Tomáš Lukáč týmto pozývame žiaka ' + meno) + ' ') + priezvisko) + ' ') + 
    datumNarodenia) + ' na prijímaciu skúšku 1. termín (2.kolo) do  odboru vzdelávania 2940M04-potravinárstvo - kvasná technológia, v škole Stredná škola pre AT, ktorá sa uskutoční dňa 25.09.2026 o 11:11 hod.  Miesto:  Stredná škola pre AT, 6.C  Váš prístupový kód: ') + 
    pristupovyKod) + '. Odporúčame si ho bezpečne uložiť. Predmety prijímacej skúšky Matematika, Slovenský jazyk a literatúra. Prineste si gaštany, budú sa stavať zvieratká. S pozdravom Tím elektronických prihlášok MŠVVaM SR Tento email bol generovaný automaticky portálom Elektronické prihlášky do škôl, ktorý je v správe Ministerstva školstva, výskumu, vývoja a mládeže Slovenskej republiky. Neodpovedajte naň.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Vygenerova pozvnky  ePrihlky/div_Generovanie 1 pozvnok bolo spusten_idsk_ba7d38'), 
    'Tento proces môže v závislosti od počtu vybraných pozvánok trvať niekoľko minút až hodín.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Vygenerova pozvnky  ePrihlky/a_Tento proces me v zvislosti od potu vybra_f585be'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/a_Prihlky_govuk-header__link'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_(nepovinn)_prijimacky-btn-zoradit-po_1f8ad8'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/label_Poda priezviska (abecedne - vzostupne_7a4950'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Sp_btn-zoradit govuk-button govuk-bu_4b9abc'))

//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Neodoslan_data-komunikacia-odoslana badge'),  'Odoslaná - Pozvánka')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/span_Meno, odbor a kd odboru_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Nastavenie termnu skky_vykonat-akciu-btn'))

'Správa o plnom počte bodov'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/a_2'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Apply_title_1'), 
    'Vygenerovať správu o plnom počte bodov')

WebUI.delay(1)

'Odoslať správu o bodoch'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

'Odoslať správu o bodoch'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

teloMailu = mail.getLastEmailText('pop.gmail.com', 'pop3', GlobalVariable.mailLogin, GlobalVariable.mailHeslo)

teloMailu = help.cleanupCidUrls(teloMailu)

teloMailu = teloMailu.replaceAll('\\r?\\n+', ' ').replaceAll('\\s+', ' ').trim()

println(teloMailu)

assert teloMailu.contains(((((((('Vážený/á pán/pani Tomáš Lukáč  žiak ' + meno) + ' ') + priezvisko) + ' ') + datumNarodenia) + 
    ' splnil podmienky na dosiahnutie plného počtu bodov z prijímacích skúšok do odboru vzdelávania 2940M04-potravinárstvo - kvasná technológia, v škole Stredná škola pre AT, ktoré mu boli udelené v systéme.  Výsledky prijímacieho konania si môžete pozrieť, keď budú dostupné pod číselným prístupovým kódom, ktorý bol žiakovi pridelený.  Váš prístupový kód: ') + 
    pristupovyKod) + '. Odporúčame si ho bezpečne uložiť. Prosím, zapíšte si ho. S pozdravom Tím elektronických prihlášok MŠVVaM SR Tento email bol generovaný automaticky portálom Elektronické prihlášky do škôl, ktorý je v správe Ministerstva školstva, výskumu, vývoja a mládeže Slovenskej republiky. Neodpovedajte naň.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/li_Prihlky_ss-riaditel-prijimacky'))

String text = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Neodoslan_data-komunikacia-odoslana badge_1'))

assert text.contains('Odoslaná - Body')

'pozvánka PDF'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/button_Akcia_govuk-button govuk-button--sec_e3fab9'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/a_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/button_Akcia_govuk-button govuk-button--sec_e3fab9'))

'plný počet bodov PDF'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/a_2'))

def den = LocalDate.now().dayOfMonth

def mesiac = LocalDate.now().monthValue

def rok = LocalDate.now().year

def hodina = LocalDateTime.now().hour

def minuta = LocalDateTime.now().minute

if(mesiac < 10)
{
	mesiac = "0"+mesiac
}

if(den < 10)
{
	den = "0"+den
}
	
if (hodina < 10) {
	hodina = "0"+hodina
}
	
if (minuta < 10) {
	minuta = "0"+minuta
}
		
	
String cestaBody = "Správa o bodoch_EDUID_910021624_"+rok+"-"+mesiac+"-"+den+"_"+hodina+"-"+minuta+".pdf"
String cestaPozvanka = "Pozvánka na PS_EDUID_910021624_"+rok+"-"+mesiac+"-"+den+"_"+hodina+"-"+minuta+".pdf"

CustomKeywords.'com.kms.katalon.keyword.pdf.PDF.compareAllPages'(pozvanka, GlobalVariable.downloadFolder+cestaPozvanka, ['xX6pSodxBqT5','Emil Hrnka 01.01.2010', meno+" "+priezvisko+" "+datumNarodenia, pristupovyKod])
CustomKeywords.'com.kms.katalon.keyword.pdf.PDF.compareAllPages'(body, GlobalVariable.downloadFolder+cestaBody, ['xX6pSodxBqT5','Emil Hrnka 01.01.2010', meno+" "+priezvisko+" "+datumNarodenia, pristupovyKod])

prihlasovanie.odhlasPouzivatela()
