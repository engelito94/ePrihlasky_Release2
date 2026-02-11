package portal;
import internal.GlobalVariable;
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint;
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase;
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData;
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject;
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject;

import com.kms.katalon.core.annotation.Keyword;
import com.kms.katalon.core.checkpoint.Checkpoint;
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords;
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords;
import com.kms.katalon.core.model.FailureHandling;
import com.kms.katalon.core.testcase.TestCase;
import com.kms.katalon.core.testdata.TestData;
import com.kms.katalon.core.testobject.TestObject;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords;
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords;
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords;

public class Helper {
	String rcToDatumNarodenia(String rc) {
		// Odstráň nepotreb né znaky (lomka, medzery, atď.)
		def cislo = rc.replaceAll(/[^0-9]/, '')

		// Minimalná dĺžka rodného čísla (min. prvých 6 čísel = dátum)
		if (!cislo || cislo.length() < 6) {
			throw new IllegalArgumentException("Nesprávny formát rodného čísla: ${rc}")
		}

		// Prvých 6 čísel = yymmDD
		def yymmdd = cislo.substring(0, 6)

		def rok  = 2009
		def mesiac = yymmdd.substring(2, 4).toInteger()
		def den  = yymmdd.substring(4, 6).toInteger()

		// Oprava mesiaca pre ženy (ak je väčší ako 12)
		if (mesiac > 12) {
			mesiac -= 50
		}

		// Určenie plného roku (podľa 20./21. storočia)
		// Jednoduchý prístup: ak rok >= 00 && <= 53 → rok = 19yy, inak 20yy
		def plnyRok = rok >= 0 && rok <= 53 ? 1900 + rok : 2000 + rok

		// Overenie validnosti dňa/mesíca/koko dní v mesiaci možno ešte pridať (tu iba zjednodušená verzia)
		try {
			def datum = Date.parse("yyyy-MM-dd", "${rok}-${mesiac}-${den}")
			return datum.format("dd.MM.yyyy")
		} catch (e) {
			throw new IllegalArgumentException("Nedá sa skonvertovať rodné číslo na platný dátum: ${rc}")
		}
	}

	boolean isMuz(String rc) {
		// Odstráň nepotrebné znaky (lomka, medzery, atď.)
		def cislo = rc.replaceAll(/[^0-9]/, '')

		// Minimalná dĺžka rodného čísla (min. prvých 4 čísel)
		if (!cislo || cislo.length() < 4) {
			throw new IllegalArgumentException("Nesprávny formát rodného čísla: ${rc}")
		}

		def mesiac = cislo.substring(2, 4).toInteger()

		// Muž: mesiac 01-12, žena: mesiac 51-62 (po úprave -50 = 01-12)
		return mesiac <= 12
	}
}