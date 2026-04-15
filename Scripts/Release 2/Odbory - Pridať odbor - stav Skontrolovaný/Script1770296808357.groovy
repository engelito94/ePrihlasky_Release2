import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration
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
import internal.GlobalVariable
import portal.Prihlasovanie

import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910014291')

'Odbory a kritériá'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/h1_Odhlsi_title main-title'), 
    'Odbory a kritériá')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/p_Odbory a kritri_sub-riaditel-odbory-subtitle'), 
    'Spravujte svoje odbory a kritériá.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/h1_Nahra rozhodnutie_title odbory-title'), 
    'Moje odbory')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/p_Moje odbory_subtitle'), 
    'Vytvorte nové odbory alebo spravujte existujúce. Odbory budú uchádzačom dostupné vo vyhľadávaní škôl a použijú ich pri podávaní prihlášok na vašu školu.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/div_close_idsk-card__heading'), 
    'Žiadne odbory v ročníku')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/div_iadne odbory v ronku_idsk-card__description'), 
    'Kliknite na tlačidlo Pridať odbor, pre doplnenie odborov ktoré máte povolené v Rezortnom informačnom systéme, alebo skopírujte tlačidlom nižšie odbory, ktoré ste použili v portáli minulý rok.')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/span_Prida odbor_material-icons-outlined'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/button_Prida odbor_btnPridatOdbor'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/span_(nepovinn)_checkmark'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/div_Uloi_title'), 
    'Pridať odbor/y')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/div_Prida odbory_description'), 
    'Označte len odbory so schválenou kapacitou na prijatie.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/button_Sp_btn-pridat-odbory govuk-button go_c64a56'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/span'), 
    'Odbory boli úspešne pridané')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/td_O8_odbor-kod'), 
    '6314N00')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/span_O8_odbor-nazov'), 
    'cestovný ruch')

'Upraviť odbor'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/button_Bez prijmacej skky_govuk-button govu_0917a3'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/input_(nepovinn)_input-modalUpravitOdborKap_547317'), 
    '10')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/span_iaci 8. ronka_checkmark'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/select_(nepovinn)_select-modalUpravitOdborD_7f569d'), 
    '20', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/select_(nepovinn)_select-modalUpravitOdborF_bc0fe5'), 
    '101', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/span_close_checkmark'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/input_(nepovinn)_input-modalUpravitOdborDua_f11a7b'), 
    '10')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/input_(nepovinn)_input-modalUpravitOdborIco_fd48d8'), 
    '31385401')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/a_IO m ma 8 slic_btn-pridat-ico'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/div_close_zamestnavatel-nazov'), 
    'DITEC, a.s.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/button_Sp_btn-uloz-zmeny govuk-button govuk_23dc44'))

//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/span_undefined kB_material-icons'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/div_Kritri na prijatie_idsk-card__content'), 
    0)

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/label_alebo ho sem potiahnite (max. 50 MB v_8d61ca'),
	priloha)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/span_check_circle_panel-text'), 
    'Dokument bol úspešne nahratý.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/div_close_warnTextBoxHeader'), 
    'Odbory sú pripravené na zverejnenie')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/a_Pred zverejnenm odborov oznate vetky odbo_745e59'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/PridatOdbor/Page_Odbory a kritri  ePrihlky/div_Odhlsi_warnTextBoxHeader'), 
    'Odbory nie sú zverejnené')

prihlasovanie.odhlasPouzivatela()