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
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

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

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Nhradn termn_data-komunikacia-neodoslana badge'), 
    'Neodoslaná')

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

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b'))

'Odoslanie pozvánky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-odoslat govuk-button govuk-_541f94'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Vygenerova pozvnky  ePrihlky/div_Vygenerova pozvnky_idsk-card__heading'), 
    'Generovanie 1 pozvánok bolo spustené.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Vygenerova pozvnky  ePrihlky/div_Generovanie 1 pozvnok bolo spusten_idsk_ba7d38'), 
    'Tento proces môže v závislosti od počtu vybraných pozvánok trvať niekoľko minút až hodín.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Vygenerova pozvnky  ePrihlky/a_Tento proces me v zvislosti od potu vybra_f585be'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/a_Prihlky_govuk-header__link'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_(nepovinn)_prijimacky-btn-zoradit-po_1f8ad8'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/label_Poda priezviska (abecedne - vzostupne_7a4950'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Sp_btn-zoradit govuk-button govuk-bu_4b9abc'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Neodoslan_data-komunikacia-odoslana badge'), 
    'Odoslaná - Pozvánka')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/span_Meno, odbor a kd odboru_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Nastavenie termnu skky_vykonat-akciu-btn'))

'Správa o plnom počte bodov'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/a_2'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Apply_title_1'), 
    'Vygenerovať správu o plnom počte bodov')

'Odoslať pozvánky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/li_Prihlky_ss-riaditel-prijimacky'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/Page_Prihlky a rozhodnutia  ePrihlky/div_Neodoslan_data-komunikacia-odoslana badge_1'), 
    'Odoslaná - Bodyinfo')

